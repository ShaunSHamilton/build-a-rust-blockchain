//! # Blockchain
//!
//! `blockchain` is a WASM module for handling a Proof of Stake blockchain.

pub mod account;
pub mod block;
pub mod chain;

use account::{Account, AccountTrait};
use chain::Chain;

use crate::chain::ChainTrait;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::*;

/// TODO: Define the global difficulty prefix to be used in the mining process.
/// Increasing the number of leading zeros in the hash of a block increases the difficulty of mining a block.
pub static DIFFICULTY_PREFIX: &str = "0";

/// TODO: Complete this enum definition. Be sure to derive the necessary implementations
/// Events that can be emitted in the `event` field of a `Transaction`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Events {
    AddAccount,
    Punish,
    Reward,
    Stake,
    Transfer(String, u64),
    Unstake,
}

/// TODO: Complete this struct definition. Be sure to derive the necessary implementations
/// A transaction describes the change which needs to be mined into a block. The transaction is associated with the `address` of an `Account`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub address: String,
    pub event: Events,
}

/// TODO: Complete this struct definition. Be sure to derive the necessary implementations
/// The current state of the Account calling the API.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeState {
    pub chain: Chain,
    pub network: Vec<String>,
    pub transactions: Vec<Transaction>,
}

/// TODO: Define `mine_block`, `validate_block`, and `initialise_chain` functions here:

/// Mines the next block onto the given chain passed in the `node_state` argument.
///
/// # Examples
///
/// ```js
/// const nodeState = {
///   chain: {
///     chain: [],
///   },
///   network: ["node_1", "node_2"],
///   transactions: [
///     {
///       event: "UpdateChain",
///       address: "node_2"
///     }
///   ],
/// };
/// const result = mine_block(nodeState);
/// ```
#[wasm_bindgen]
pub fn mine_block(node_state: JsValue) -> Result<JsValue, JsError> {
    let node_state: NodeState = node_state.into_serde()?;
    let mut chain = node_state.chain;
    let mut errors: Vec<String> = vec![];

    let mut unique_nodes_before: Vec<Account> = vec![];
    for transaction in node_state.transactions.iter() {
        if let Some(node) = chain.get_account_by_address(&transaction.address) {
            if !unique_nodes_before
                .iter()
                .any(|n| n.address == node.address)
            {
                let node = node.clone();
                unique_nodes_before.push(node);
            }
        }
    }

    let mut unique_nodes_final = unique_nodes_before.clone();

    for transaction in node_state.transactions.iter() {
        if let Some(node) = unique_nodes_before
            .iter_mut()
            .find(|n| n.address == transaction.address)
        {
            match &transaction.event {
                Events::Unstake => {
                    if node.can_unstake() {
                        node.staked -= 1;
                    } else {
                        errors.push(format!("'{}' cannot unstake", node.address));
                    }
                }
                Events::Stake => {
                    if node.can_stake() {
                        node.staked += 1;
                    } else {
                        errors.push(format!("'{}' cannot stake", node.address));
                    }
                }
                Events::Transfer(to, amount) => {
                    if &node.address == to {
                        errors.push(format!("'{}' cannot transfer to itself", node.address));
                    } else {
                        if node.can_transfer(&amount) {
                            // Check if recipient is in `unique_nodes`
                            // Else, check if recipient is in `chain`
                            if let Some(recipient) =
                                unique_nodes_final.iter_mut().find(|n| &n.address == to)
                            {
                                recipient.tokens += amount;
                                node.tokens -= amount;
                            } else {
                                if let Some(recipient) = chain.get_account_by_address(&to) {
                                    let mut to_node = recipient.clone();
                                    to_node.tokens += amount;
                                    unique_nodes_final.push(to_node);
                                    node.tokens -= amount;
                                } else {
                                    errors.push(format!("Recipient '{}' not found in chain", to));
                                }
                            }
                        } else {
                            errors.push(format!(
                                "'{}' cannot transfer {} tokens",
                                node.address, amount
                            ));
                        }
                    }
                }
                _ => {
                    errors.push(format!(
                        "'{}' transacted with an invalid event",
                        node.address
                    ));
                }
            };
        } else {
            match transaction.event {
                Events::AddAccount => {
                    // Add node to chain
                    unique_nodes_final.push(Account::new(&transaction.address));
                }
                _ => {
                    errors.push(format!("'{}' not found in chain", transaction.address));
                }
            };
        }
    }

    // Update `unique_nodes_final` with `unique_nodes_before`
    for node in unique_nodes_before.iter() {
        if let Some(index) = unique_nodes_final
            .iter()
            .position(|n| n.address == node.address)
        {
            unique_nodes_final[index] = node.clone();
        }
    }

    if errors.len() == node_state.transactions.len() || unique_nodes_final.len() == 0 {
        return Err(JsError::new("Invalid transactions. No change in chain"));
    }
    chain.mine_block(unique_nodes_final, node_state.network);
    // let response = Res { chain, errors };
    Ok(JsValue::from_serde(&(chain, errors))?)
}

/// Validates whether the provided `chain` argument is valid for the latest two blocks in the chain.
///
/// # Examples
///
/// ```js
/// const chain = {
///   chain: [
///    {
///     id: 0,
///     hash: "01101101",
///     previous_hash: "genesis",
///     timestamp: 123456789,
///     data: [
///       {
///         name: "node_1",
///         tokens: 20,
///         staked: 0,
///         reputation: 1,
///         racks: 0
///       }],
///     nonce: 0,
///     next_miner: "node_1",
///     next_validators: ["node_2"]
///    }],
///   network: ["node_1", "node_2"]
/// };
/// const isChainValid = handle_validate(chain);
/// assert.equal(isChainValid, true);
/// ```
///
/// # Errors
///
/// If `chain` argument is not deserialisable into type `Chain`, a `JsError` is thrown.
#[wasm_bindgen]
pub fn validate_block(chain: JsValue) -> Result<bool, JsError> {
    let chain: Chain = chain.into_serde()?;
    if chain.len() < 2 {
        return Err(JsError::new("Chain is too short"));
    }
    if let Some(previous_block) = chain.get(chain.len() - 2) {
        if let Some(last_block) = chain.get_last_block() {
            Ok(Account::validate_block(&last_block, previous_block))
        } else {
            Err(JsError::new("Unable to get latest block from chain"))
        }
    } else {
        Err(JsError::new("Unhandled error"))
    }
}

/// Initialise a new blockchain, and returns the corresponding chain.
/// This is only to be called by the first Account starting the network.
#[wasm_bindgen]
pub fn initialise_chain(address: String) -> Result<JsValue, JsError> {
    let mut chain: Chain = Chain::new();

    // Create and mine genesis block
    let genesis_node = Account::new(&address);
    let genesis_address = genesis_node.address.clone();
    let data = vec![genesis_node];
    let network = vec![genesis_address];
    chain.mine_block(data, network);

    Ok(JsValue::from_serde(&chain)?)
}

/// TODO: Complete this function:
/// Takes a hash slice, and returns the binary representation.
pub fn hash_to_binary(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}

/// TODO: Complete this function:
/// Uses `Sha256` to calculate the hash from a `serde_json::Value` of the input arguments.
pub fn calculate_hash(
    data: &Vec<Account>,
    id: u64,
    next_miner: &str,
    next_validators: &Vec<String>,
    nonce: u64,
    previous_hash: &str,
    timestamp: u64,
) -> Vec<u8> {
    let data = serde_json::json!({
        "id": id,
        "previous_hash": previous_hash,
        "data": data,
        "timestamp": timestamp,
        "nonce": nonce,
        "next_miner": next_miner,
        "next_validators": next_validators,
    });
    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()
}

// DO NOT EDIT TESTS
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn calculate_hash_works() {
        let data = vec![Account::new("Shaun")];
        let hash = calculate_hash(&data, 1, "test", &vec!["test".to_string()], 1, "test", 1);
        let expected_hash = [
            104, 101, 236, 24, 3, 47, 224, 77, 47, 52, 255, 237, 205, 181, 209, 162, 180, 139, 160,
            115, 147, 254, 129, 29, 245, 49, 171, 8, 28, 29, 116, 198,
        ];
        assert_eq!(hash.len(), 32);
        assert_eq!(hash, expected_hash);
        let data = vec![Account::new("Tom")];
        let hash = calculate_hash(
            &data,
            1,
            "Mrugesh",
            &vec!["Shaun".to_string()],
            1,
            "Quincy",
            1,
        );
        let expected_hash = [
            5, 115, 102, 222, 65, 49, 98, 111, 42, 138, 233, 77, 213, 12, 96, 154, 168, 222, 27,
            251, 144, 8, 233, 164, 50, 174, 141, 146, 8, 145, 8, 72,
        ];
        assert_eq!(hash.len(), 32);
        assert_eq!(hash, expected_hash);
    }
    #[test]
    fn hash_to_binary_works() {
        let hash = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let hash_str = hash_to_binary(&hash);
        assert_eq!(
            hash_str,
            "01101110010111011110001001101010111100110111101111"
        );
        assert_eq!(hash_str.len(), 50);
        let hash = [
            5, 115, 102, 222, 65, 49, 98, 111, 42, 138, 233, 77, 213, 12, 96, 154, 168, 222, 27,
            251, 144, 8, 233, 164, 50, 174, 141, 146, 8, 145, 8, 72,
        ];
        let hash_str = hash_to_binary(&hash);
        assert_eq!(
            hash_str,
            "10111100111100110110111101000001110001110001011011111010101000101011101001100110111010101110011000001001101010101000110111101101111111011100100001000111010011010010011001010101110100011011001001010001001000110001001000"
        );
        assert_eq!(hash_str.len(), 218);
    }
}
