//! # Blockchain
//!
//! `blockchain` is a WASM module for handling a Proof of Stake blockchain.

pub mod account;
pub mod block;
pub mod chain;

// TODO: Import necessary modules

/// TODO: Define the global difficulty prefix to be used in the mining process.
/// Increasing the number of leading zeros in the hash of a block increases the difficulty of mining a block.

/// TODO: Complete this enum definition. Be sure to derive the necessary implementations
/// Events that can be emitted in the `event` field of a `Transaction`.
pub enum Events {}

/// TODO: Complete this struct definition. Be sure to derive the necessary implementations
/// A transaction describes the change which needs to be mined into a block. The transaction is associated with the `address` of an `Account`.
pub struct Transaction {}

/// TODO: Complete this struct definition. Be sure to derive the necessary implementations
/// The current state of the Account calling the API.
pub struct NodeState {}

/// TODO: Define `mine_block`, `validate_block`, and `initialise_chain` functions below:

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

/// Initialise a new blockchain, and returns the corresponding chain.
/// This is only to be called by the first Account starting the network.

/// TODO: Complete this function:
/// Takes a hash slice, and returns the binary representation.
pub fn hash_to_binary(hash: &[u8]) -> String {
    unimplemented!()
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
    unimplemented!()
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
