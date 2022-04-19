//! # Account
//!
//! A node is a piece of data that is stored in a `BLock` on the blockchain.

use serde::{Deserialize, Serialize};

use crate::{block::Block, calculate_hash, hash_to_binary, DIFFICULTY_PREFIX};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub address: String,
    pub staked: u64,
    pub tokens: u64,
}

impl Account {
    /// Creates a new Account with the given name.
    ///
    /// # Examples
    ///
    /// ```
    /// let node = Account::new();
    /// assert_eq!(node.address, "Camper");
    /// ```
    pub fn new(address: String) -> Self {
        Self {
            address,
            staked: 0,
            tokens: 20,
        }
    }

    /// Check if Account can afford a server rack
    pub fn can_buy_rack(&self) -> bool {
        self.tokens - self.staked >= 10
    }
    /// Check if a Account can stake, by checking if it has any unstaked tokens
    pub fn can_stake(&self) -> bool {
        self.tokens > self.staked
    }
    /// Check if a Account can unstake, by checking if it has any staked tokens
    pub fn can_unstake(&self) -> bool {
        self.staked > 0
    }
    pub fn can_transfer(&self, amount: &u64) -> bool {
        self.tokens >= *amount
    }
    /// Check if a Account can be punished, by checking if it has any tokens and reputation
    pub fn can_punish(&self) -> bool {
        self.tokens > 0
    }
    /// Calculates the miner weight of Account
    pub fn weight_as_miner(&self) -> u64 {
        self.staked
    }
    /// Calculates the validator weight of Account
    pub fn weight_as_validator(&self) -> u64 {
        self.staked
    }

    /// Validates if two adjacent blocks have been correctly mined
    pub fn validate_block(block: &Block, previous_block: &Block) -> bool {
        if block.previous_hash != previous_block.hash {
            println!("block with id: {} has wrong previous hash", block.id);
            return false;
        } else if !&block.hash.starts_with(DIFFICULTY_PREFIX) {
            println!("block with id: {} has invalid difficulty", block.id);
            return false;
        } else if block.id != previous_block.id + 1 {
            println!(
                "block with id: {} is not the next block after the latest: {}",
                block.id, previous_block.id
            );
            return false;
        } else if hash_to_binary(&calculate_hash(
            &block.data,
            block.id,
            &block.next_miner,
            &block.next_validators,
            block.nonce,
            &block.previous_hash,
            block.timestamp,
        )) != block.hash
        {
            println!("block with id: {} has invalid hash", block.id);
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_always_creates_same_node() {
        let address = "example".to_string();
        let node1 = Account::new(address.clone());
        let node2 = Account::new(address);
        assert_eq!(node1.address, node2.address);
        assert_eq!(node1.staked, node2.staked);
        assert_eq!(node1.tokens, node2.tokens);
    }
    #[test]
    fn cannot_buy_rack_when_all_staked() {
        let node = _fixture_nodes().0;
        assert!(!node.can_buy_rack());
    }
    #[test]
    fn can_buy_rack_when_all_unstaked() {
        let node = _fixture_nodes().1;
        assert!(node.can_buy_rack());
    }
    #[test]
    fn cannot_buy_rack_when_no_tokens() {
        let node = _fixture_nodes().2;
        assert!(!node.can_buy_rack());
    }
    #[test]
    fn cannot_stake_when_all_staked() {
        let node = _fixture_nodes().0;
        assert!(!node.can_stake());
    }
    #[test]
    fn can_stake_when_all_unstaked() {
        let node = _fixture_nodes().1;
        assert!(node.can_stake());
    }
    #[test]
    fn cannot_stake_when_no_tokens() {
        let node = _fixture_nodes().2;
        assert!(!node.can_stake());
    }
    #[test]
    fn can_unstake_when_all_staked() {
        let node = _fixture_nodes().0;
        assert!(node.can_unstake());
    }
    #[test]
    fn cannot_unstake_when_all_unstaked() {
        let node = _fixture_nodes().1;
        assert!(!node.can_unstake());
    }
    #[test]
    fn cannot_unstake_when_no_tokens() {
        let node = _fixture_nodes().2;
        assert!(!node.can_unstake());
    }
    #[test]
    fn all_staked_miner_weight() {
        let node = _fixture_nodes().0;
        assert_eq!(node.weight_as_miner(), 100);
    }
    #[test]
    fn all_unstaked_miner_weight() {
        let node = _fixture_nodes().1;
        assert_eq!(node.weight_as_miner(), 0);
    }
    #[test]
    fn no_tokens_miner_weight() {
        let node = _fixture_nodes().2;
        assert_eq!(node.weight_as_miner(), 0);
    }
    #[test]
    fn all_staked_validator_weight() {
        let node = _fixture_nodes().0;
        assert_eq!(node.weight_as_validator(), 100);
    }
    #[test]
    fn all_unstaked_validator_weight() {
        let node = _fixture_nodes().1;
        assert_eq!(node.weight_as_validator(), 0);
    }
    #[test]
    fn no_tokens_validator_weight() {
        let node = _fixture_nodes().2;
        assert_eq!(node.weight_as_validator(), 0);
    }
    #[test]
    fn invalidate_block_unequal_previous_hash() {
        let previous_block = _fixture_blocks().0;
        let mut block = _fixture_blocks().1;
        block.previous_hash = block.previous_hash.replace("1", "0");
        assert!(!Account::validate_block(&block, &previous_block));
    }
    #[test]
    fn invalidate_block_hash_not_start_with_difficulty() {
        let previous_block = _fixture_blocks().0;
        let mut block = _fixture_blocks().1;
        block.hash = block.previous_hash.replace("0", "1"); //"011111101111000110011110001110011001111011011010011011000101110010100101001001111001110001011010111000010011000010000100101000011010111110001110010011110101011000011101011110011001110010111001011011011111111010110100000".to_string();
        assert!(!Account::validate_block(&block, &previous_block));
    }
    #[test]
    fn invalidate_block_id_not_incremented() {
        let previous_block = _fixture_blocks().0;
        let mut block = _fixture_blocks().1;
        block.id = previous_block.id;
        assert!(!Account::validate_block(&block, &previous_block));
    }
    #[test]
    fn invalidate_encoded_hash_not_correct() {
        let previous_block = _fixture_blocks().0;
        let mut block = _fixture_blocks().1;
        block.hash = "001111101111000110011110001110011001111011011010011011000101110010100101001001111001110001011010111000010011000010000100101000011010111110001110010011110101011000011101011110011001110010111001011011011111111010110100000".to_string();
        assert!(!Account::validate_block(&block, &previous_block));
    }
    #[test]
    fn validate_block_correct() {
        let previous_block = _fixture_blocks().0;
        let block = _fixture_blocks().1;

        assert!(Account::validate_block(&block, &previous_block));
    }

    fn _fixture_nodes() -> (Account, Account, Account) {
        let all_staked = Account {
            address: "Shaun".to_string(),
            staked: 100,
            tokens: 100,
        };
        let all_unstaked = Account {
            address: "Tom".to_string(),
            staked: 0,
            tokens: 100,
        };
        let no_tokens = Account {
            address: "Quincy".to_string(),
            staked: 0,
            tokens: 0,
        };
        (all_staked, all_unstaked, no_tokens)
    }
    fn _fixture_blocks() -> (Block, Block) {
        let id = 0;
        let previous_hash = String::new();
        let data = vec![
            Account {
                address: "Camper".to_string(),
                staked: 0,
                tokens: 10,
            },
            Account {
                address: "Tom".to_string(),
                staked: 0,
                tokens: 10,
            },
            Account {
                address: "Mrugesh".to_string(),
                staked: 0,
                tokens: 10,
            },
        ];
        let nonce = 83;
        let next_miner = "Camper".to_string();
        let next_validators = vec![
            "Camper".to_string(),
            "Camper".to_string(),
            "Camper".to_string(),
        ];
        let timestamp = 1648987026;
        let hash = hash_to_binary(&calculate_hash(
            &data,
            id,
            &next_miner,
            &next_validators,
            nonce,
            &previous_hash,
            timestamp,
        ));
        let genesis = Block {
            id,
            hash: hash.clone(),
            previous_hash,
            timestamp,
            data,
            nonce,
            next_miner,
            next_validators,
        };

        // ----------------------------
        let id = 1;
        let previous_hash = hash;
        let timestamp = 1650301013;
        let data = vec![Account {
            address: "Ahmad".to_string(),
            staked: 0,
            tokens: 10,
        }];
        let nonce = 172;
        let next_miner = "Mrugesh".to_string();
        let next_validators = vec!["Mrugesh".to_string()];
        let hash = hash_to_binary(&calculate_hash(
            &data,
            id,
            &next_miner,
            &next_validators,
            nonce,
            &previous_hash,
            timestamp,
        ));

        // Used to update test
        // use crate::{Chain, ChainTrait};
        // let mut chain = Chain::new();
        // chain.push(genesis.clone());
        // chain.mine_block(data.clone(), next_validators.clone());
        // println!("{:?}", chain);

        let block = Block {
            id,
            hash,
            previous_hash,
            timestamp,
            data,
            nonce,
            next_miner,
            next_validators,
        };

        (genesis, block)
    }
}
