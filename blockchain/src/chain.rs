//! # Chain
//!
//! A chain represents the main data of the blockchain, and is passed in full between Nodes.

use rand::Rng;
// use serde::{Deserialize, Serialize};

use crate::{block::Block, calculate_hash, hash_to_binary, node::Node, DIFFICULTY_PREFIX};

/// The chain consists of the immutable `chain` data.
pub type Chain = Vec<Block>;

impl ChainTrait for Chain {
    fn new() -> Self {
        vec![]
    }
    fn get_last_block(&self) -> Option<Block> {
        match self.last() {
            Some(block) => Some(block.clone()),
            None => None,
        }
    }
    fn get_next_miner(&self) -> String {
        let mut nodes: Vec<&Node> = self.get_nodes();

        nodes.sort_by(|a, b| a.weight_as_miner().cmp(&b.weight_as_miner()));
        let cumulative_weight = nodes
            .iter()
            .fold(0, |acc, node| acc + node.weight_as_miner());
        let cumulative_weights = nodes
            .iter()
            .map(|node| node.weight_as_miner() as f64 / cumulative_weight as f64)
            .collect::<Vec<f64>>();

        let rand_num = rand::thread_rng().gen::<f64>();
        let mut ind = 0;
        for (i, c_m) in cumulative_weights.iter().enumerate() {
            ind = i;
            if c_m > &rand_num {
                break;
            }
        }
        let next_miner = match nodes.get(ind) {
            Some(node) => node.address.clone(),
            None => "Camper".to_string(),
        };

        next_miner
    }
    fn get_next_validators(&self, next_miner: &String, network: Vec<String>) -> Vec<String> {
        let num_of_nodes = network.len();
        let next_miner_staked = match self.get_node_by_address(next_miner) {
            Some(node) => node.staked,
            None => 0,
        };

        let mut nodes = self.get_nodes();

        let mut max_staked = 0;
        for node in nodes.iter() {
            if node.staked > max_staked {
                max_staked = node.staked;
            }
        }

        // The greater the staked tokens, the fewer validators are needed
        let num_needed_validators = (max_staked - next_miner_staked) as usize;
        let num_needed_validators = if num_needed_validators > num_of_nodes {
            num_of_nodes
        } else {
            num_needed_validators + 1
        };

        nodes.sort_by(|a, b| a.weight_as_validator().cmp(&b.weight_as_validator()));

        let cumulative_weight = nodes
            .iter()
            .fold(0, |acc, node| acc + node.weight_as_validator());
        let cumulative_weights = nodes
            .iter()
            .map(|node| node.weight_as_validator() as f64 / cumulative_weight as f64)
            .collect::<Vec<f64>>();

        println!(
            "Cumulative Weight: {:?}\nCumulative Weights: {:?}\n\n",
            cumulative_weight, cumulative_weights
        );
        let mut next_validators = vec![];
        for _ in 0..num_needed_validators {
            let rand_num = rand::thread_rng().gen::<f64>();
            let mut ind = 0;
            for (i, c_m) in cumulative_weights.iter().enumerate() {
                if c_m > &rand_num {
                    break;
                }
                ind = i;
            }
            let validator = match nodes.get(ind) {
                Some(node) => node.address.clone(),
                None => network[0].clone(),
            };
            next_validators.push(validator);
        }
        next_validators
    }
    fn get_node_by_address(&self, address: &str) -> Option<&Node> {
        // Search Chain data in reverse
        for block in self.iter().rev() {
            for node in block.data.iter() {
                if node.address == address {
                    return Some(node);
                }
            }
        }
        None
    }
    fn get_nodes(&self) -> Vec<&Node> {
        let mut nodes = vec![];
        for block in self.iter().rev() {
            for node in block.data.iter() {
                // If node.name is not in nodes, add it
                if !nodes.iter().any(|n: &&Node| n.address == node.address) {
                    nodes.push(node);
                }
            }
        }
        nodes
    }
    fn mine_block(&mut self, data: Vec<Node>, network: Vec<String>) {
        println!("\nMining Block...");
        let mut nonce = 0;

        let id = self.len() as u64;
        let next_miner = self.get_next_miner();
        let next_validators = self.get_next_validators(&next_miner, network);
        let previous_hash = match self.get_last_block() {
            Some(block) => block.hash.clone(),
            None => "".to_string(),
        };
        let timestamp = chrono::Utc::now().timestamp() as u64;
        loop {
            if nonce % 100_000 == 0 {
                println!("Trying nonce: {}", nonce);
            }

            let hash = calculate_hash(
                &data,
                id,
                &next_miner,
                &next_validators,
                nonce,
                &previous_hash,
                timestamp,
            );
            let bin_hash = hash_to_binary(&hash);
            if bin_hash.starts_with(DIFFICULTY_PREFIX) {
                let new_block = Block {
                    id,
                    hash: bin_hash,
                    previous_hash,
                    timestamp,
                    data,
                    nonce,
                    next_miner,
                    next_validators,
                };
                self.push(new_block);
                break;
            }
            nonce += 1;
        }
    }
}

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
    /// If chain is initialised by Node:
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

    /// Applies weighted, random selection to all `Node`s in the `Chain` to determine the `next_miner` for the next block.
    ///
    /// **Note:** Defaults to returning `"Camper"`, if no `Node`s are present in the `Chain`.
    ///
    /// # Examples
    ///
    /// ```
    /// let chain = Chain::new();
    /// let next_miner = chain.get_next_miner();
    /// assert_eq!(next_miner, String::from("Camper"));
    /// ```
    fn get_next_miner(&self) -> String;

    /// Applies weighted, random selection to all `Node`s in the `Chain` to determine the `next_validators` for the next block.
    fn get_next_validators(&self, next_miner: &String, network: Vec<String>) -> Vec<String>;

    /// Returns the `Node` with the given `name` if it exists in the `Chain`. Otherwise, returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut chain = Chain::new();
    /// assert!(chain.get_node_by_name("Camper").is_none());
    /// ```
    fn get_node_by_address(&self, name: &str) -> Option<&Node>;

    /// Returns a `Vec` of all `Node`s in the `Chain`.
    fn get_nodes(&self) -> Vec<&Node>;

    /// Mines the given `data` into a new `Block` on the `Chain`.
    fn mine_block(&mut self, data: Vec<Node>, network: Vec<String>);
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn get_node_by_address_returns_none_when_node_is_not_in_chain() {
        let chain = _fixture_chain();
        assert!(chain.get_node_by_address("node_not_in_chain").is_none());
    }
    #[test]
    fn get_node_by_address_returns_node_when_node_is_in_chain() {
        let chain = _fixture_chain();
        assert!(chain.get_node_by_address("Camper").is_some());
    }
    #[test]
    fn get_nodes_returns_all_nodes_in_chain() {
        let chain = _fixture_chain();
        let nodes = chain.get_nodes();
        assert_eq!(nodes.len(), 4);
    }
    #[test]
    fn mine_block_does_not_panic() {
        let mut chain = _fixture_chain();
        let network = vec![String::from("node_1"), String::from("node_2")];
        chain.mine_block(vec![Node::new("node_3".to_string())], network);
        assert_eq!(chain.len(), 3);
    }

    fn _fixture_chain() -> Chain {
        let mut chain = Chain::new();

        let mut camper = Node::new("Camper".to_string());
        camper.tokens = 10;
        camper.staked = 5;
        let mut tom = Node::new("Tom".to_string());
        tom.tokens = 20;
        tom.staked = 10;
        let mut mrugesh = Node::new("Mrugesh".to_string());
        mrugesh.tokens = 100;
        mrugesh.staked = 80;
        let mut ahmad = Node::new("Ahmad".to_string());
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
