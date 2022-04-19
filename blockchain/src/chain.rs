//! # Chain
//!
//! A chain represents the main data of the blockchain, and is passed in full between Accounts.

/// TODO: Define the `Chain` type.
/// The chain consists of the immutable `Block` data.

/// TODO: Implement the `ChainTrait` for the `Chain` type.

/// The chain trait defines the methods that a `Chain` must implement.
///
/// **Note:** Do not change this trait definition.
pub trait ChainTrait {
    /// Creates a `Chain` with empty `chain` and `network` data.
    fn new() -> Self;

    /// Returns the last block of the current `Chain`, if it exists. Otherwise, returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut chain = Chain::new();
    /// let last_block = chain.last_block();
    /// assert!(last_block.is_none());
    /// ```
    ///
    /// If chain is initialised by Account:
    ///
    /// ```javascript
    /// const chain = initialise("node_1");
    /// assert.equal(chain.chain.length, 1);
    /// ```
    ///
    /// ```
    /// assert_eq!(chain.last_block().unwrap().id, 0);
    /// ```
    fn get_last_block(&self) -> Option<Block>;

    /// Applies weighted, random selection to all `Account`s in the `Chain` to determine the `next_miner` for the next block.
    ///
    /// **Note:** Defaults to returning `"Camper"`, if no `Account`s are present in the `Chain`.
    ///
    /// # Examples
    ///
    /// ```
    /// let chain = Chain::new();
    /// let next_miner = chain.get_next_miner();
    /// assert_eq!(next_miner, String::from("Camper"));
    /// ```
    fn get_next_miner(&self) -> String;

    /// Applies weighted, random selection to all `Account`s in the `Chain` to determine the `next_validators` for the next block.
    fn get_next_validators(&self, next_miner: &String, network: Vec<String>) -> Vec<String>;

    /// Returns the `Account` with the given `name` if it exists in the `Chain`. Otherwise, returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut chain = Chain::new();
    /// assert!(chain.get_node_by_name("Camper").is_none());
    /// ```
    fn get_account_by_address(&self, name: &str) -> Option<&Account>;

    /// Returns a `Vec` of all `Account`s in the `Chain`.
    fn get_accounts(&self) -> Vec<&Account>;

    /// Mines the given `data` into a new `Block` on the `Chain`.
    fn mine_block(&mut self, data: Vec<Account>, network: Vec<String>);
}

// DO NOT EDIT TESTS
#[cfg(test)]
mod tests {
    use crate::account::AccountTrait;

    use super::*;
    #[test]
    fn chain_is_vec_of_block() {
        // This test must just compile.
        let block = Block {
            id: 0,
            hash: "".to_string(),
            previous_hash: "".to_string(),
            timestamp: 0,
            data: vec![],
            nonce: 0,
            next_miner: "".to_string(),
            next_validators: vec![],
        };
        let _chain: Chain = vec![block];
    }
    #[test]
    fn chain_implements_chain_trait() {
        // This test must just compile.
        let chain: Chain = Chain::new();
        let _last_block = chain.get_last_block();
        let next_miner = chain.get_next_miner();
        let _next_validators = chain.get_next_validators(&next_miner, vec!["Camper".to_string()]);
        let _node = chain.get_account_by_address("");
        let _nodes = chain.get_accounts();
        type FuncTest = fn(&mut Chain, Vec<Account>, Vec<String>) -> ();
        let _func_test: FuncTest = Chain::mine_block;
    }
    #[test]
    fn new_chain_returns_empty_vec() {
        let chain: Chain = Chain::new();
        assert_eq!(chain.len(), 0);
    }
    #[test]
    fn get_last_block_returns_none_when_chain_is_empty() {
        let chain: Chain = Chain::new();
        assert!(chain.get_last_block().is_none());
    }
    #[test]
    fn get_last_block_returns_last_block_when_chain_is_not_empty() {
        let chain = _fixture_chain();
        let last_block = chain.get_last_block();
        assert!(last_block.is_some());
        let last_block = last_block.unwrap();
        assert_eq!(last_block.id, 1);
        assert!(last_block.next_miner.is_ascii());
    }
    #[test]
    fn get_next_miner_can_return_different_miner_when_chain_is_not_empty() {
        let chain = _fixture_chain();
        // run 100 times, break if miner is different
        let mut i = 0;
        let a = loop {
            let previous_miner = chain.get_last_block().unwrap().next_miner.clone();
            let next_miner = chain.get_next_miner();
            if previous_miner != next_miner {
                break true;
            }
            if i == 99 {
                break false;
            }
            i += 1;
        };
        assert!(a);
    }
    #[test]
    fn get_next_validators_can_return_different_validators_when_chain_is_not_empty() {
        let chain = _fixture_chain();
        // run 100 times, break if validators are different
        let mut i = 0;
        let a = loop {
            let network = vec![String::from("node_1"), String::from("node_2")];
            let previous_validators = chain.get_last_block().unwrap().next_validators.clone();
            let next_validators = chain.get_next_validators(&chain.get_next_miner(), network);
            if previous_validators != next_validators {
                break true;
            }
            if i == 99 {
                break false;
            }
            i += 1;
        };
        assert!(a);
    }
    #[test]
    fn get_account_by_address_returns_none_when_node_is_not_in_chain() {
        let chain = _fixture_chain();
        assert!(chain.get_account_by_address("node_not_in_chain").is_none());
    }
    #[test]
    fn get_account_by_address_returns_node_when_node_is_in_chain() {
        let chain = _fixture_chain();
        assert!(chain.get_account_by_address("Camper").is_some());
    }
    #[test]
    fn get_accounts_returns_all_nodes_in_chain() {
        let chain = _fixture_chain();
        let nodes = chain.get_accounts();
        assert_eq!(nodes.len(), 4);
    }
    #[test]
    fn mine_block_does_not_panic() {
        let mut chain = _fixture_chain();
        let network = vec![String::from("node_1"), String::from("node_2")];
        chain.mine_block(vec![Account::new("node_3")], network);
        assert_eq!(chain.len(), 3);
    }

    fn _fixture_chain() -> Chain {
        let mut chain = Chain::new();

        let mut camper = Account::new("Camper");
        camper.tokens = 10;
        camper.staked = 5;
        let mut tom = Account::new("Tom");
        tom.tokens = 20;
        tom.staked = 10;
        let mut mrugesh = Account::new("Mrugesh");
        mrugesh.tokens = 100;
        mrugesh.staked = 80;
        let mut ahmad = Account::new("Ahmad");
        ahmad.tokens = 30;
        ahmad.staked = 22;

        let data = vec![camper, tom, mrugesh];
        let mut network: Vec<String> = data.iter().map(|node| node.address.clone()).collect();

        chain.mine_block(data, network.clone());

        network.push(ahmad.address.clone());
        let data = vec![ahmad];

        chain.mine_block(data, network);
        chain
    }
}
