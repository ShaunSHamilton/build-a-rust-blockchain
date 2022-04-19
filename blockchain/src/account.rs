//! # Account
//!
//! An account is a piece of data that is stored in a `BLock` on the blockchain.

/// TODO: Complete the struct definition. Be sure to derive the necessary implementations.
pub struct Account {}

/// TODO: Implement the `AccountTrait` for the `Account` struct.

/// The account trait defines the methods that an `Account` must implement.
///
/// **Note:** Do not change this trait definition.
pub trait AccountTrait {
    /// Creates a new Account with the given name.
    ///
    /// # Examples
    ///
    /// ```
    /// let node = Account::new();
    /// assert_eq!(node.address, "Camper");
    /// ```
    fn new(address: &str) -> Self;
    /// Check if Account can afford a server rack
    ///
    /// **Note:** Racks cost 10 tokens.
    fn can_buy_rack(&self) -> bool;
    /// Check if a Account can stake, by checking if it has any unstaked tokens
    fn can_stake(&self) -> bool;
    /// Check if a Account can unstake, by checking if it has any staked tokens
    fn can_unstake(&self) -> bool;
    /// Check if a Account can transfer tokens, by checking if it has enough tokens
    fn can_transfer(&self, amount: &u64) -> bool;
    /// Check if a Account can be punished, by checking if it has any tokens
    fn can_punish(&self) -> bool;
    /// Calculates the miner weight of Account
    ///
    /// **Note:** Weight is equal to the number of staked tokens.
    fn weight_as_miner(&self) -> u64;
    /// Calculates the validator weight of Account
    ///
    /// **Note:** Weight is equal to the number of staked tokens.
    fn weight_as_validator(&self) -> u64;
    /// Validates if two adjacent blocks have been correctly mined
    fn validate_block(block: &Block, previous_block: &Block) -> bool;
}

// DO NOT EDIT TESTS
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn account_has_correct_fields() {
        // This test must just compile.
        let _account = Account {
            address: "example".to_string(),
            staked: 0u64,
            tokens: 20u64,
        };
    }
    #[test]
    fn account_implements_account_trait() {
        // This test must just compile.
        let account = Account::new("example");
        let _can_buy_rack: bool = account.can_buy_rack();
        let _can_stake: bool = account.can_stake();
        let _can_unstake: bool = account.can_unstake();
        let _can_transfer: bool = account.can_transfer(&10u64);
        let _can_punish: bool = account.can_punish();
        let _weight_as_miner: u64 = account.weight_as_miner();
        let _weight_as_validator: u64 = account.weight_as_validator();
        type FuncTest = fn(&Block, &Block) -> bool;
        let _validate_block_test: FuncTest = Account::validate_block;
    }
    #[test]
    fn new_always_creates_same_account() {
        let address = "example".to_string();
        let node1 = Account::new(&address);
        let node2 = Account::new(&address);
        assert_eq!(node1.address, node2.address);
        assert_eq!(node1.staked, node2.staked);
        assert_eq!(node1.tokens, node2.tokens);
    }
    #[test]
    fn new_account_tests() {
        let address = "example".to_string();
        let node = Account::new(&address);
        let second_addr = "example2".to_string();
        let second_node = Account::new(&second_addr);
        assert_eq!(
            node.address, address,
            "The given address should be used to create a new account"
        );
        assert_eq!(
            second_node.address, second_addr,
            "The given address should be used to create a new account"
        );
        assert_eq!(
            node.staked, 0,
            "A new account should start with 0 staked tokens"
        );
        assert_eq!(node.tokens, 20, "A new account should start with 20 tokens");
    }
    #[test]
    fn can_buy_rack_tests() {
        let mut account = Account::new("example");
        account.tokens = 10;
        assert!(
            account.can_buy_rack(),
            "An account with 10 tokens should be able to buy a rack"
        );
        account.tokens = 9;
        assert!(
            !account.can_buy_rack(),
            "An account with 9 tokens should not be able to buy a rack"
        );
        account.tokens = 19;
        account.staked = 10;
        assert!(
            !account.can_buy_rack(),
            "An account with 19 tokens and 10 staked tokens should not be able to buy a rack"
        );
        account.tokens = 20;
        account.staked = 10;
        assert!(
            account.can_buy_rack(),
            "An account with 20 tokens and 10 staked tokens should be able to buy a rack"
        );
    }
    #[test]
    fn can_stake_tests() {
        let mut account = Account::new("example");
        account.tokens = 1;
        assert!(
            account.can_stake(),
            "An account with 1 token should be able to stake"
        );
        account.tokens = 0;
        assert!(
            !account.can_stake(),
            "An account with 0 tokens should not be able to stake"
        );
        account.tokens = 11;
        account.staked = 10;
        assert!(
            account.can_stake(),
            "An account with 11 tokens and 10 staked tokens should be able to stake"
        );
        account.tokens = 10;
        account.staked = 10;
        assert!(
            !account.can_stake(),
            "An account with 10 tokens and 10 staked tokens should not be able to stake"
        );
    }
    #[test]
    fn can_unstake_tests() {
        let mut account = Account::new("example");
        account.tokens = 1;
        account.staked = 1;
        assert!(
            account.can_unstake(),
            "An account with 1 token and 1 staked token should be able to unstake"
        );
        account.staked = 0;
        assert!(
            !account.can_unstake(),
            "An account with 0 staked should not be able to unstake"
        );
    }
    #[test]
    fn can_transfer_tests() {
        let mut account = Account::new("example");
        account.tokens = 10;
        assert!(
            account.can_transfer(&10u64),
            "An account with 10 tokens should be able to transfer 10 tokens"
        );
        assert!(
            !account.can_transfer(&11u64),
            "An account with 10 tokens should not be able to transfer 11 tokens"
        );
        account.tokens = 0;
        assert!(
            !account.can_transfer(&1u64),
            "An account with 0 tokens should not be able to transfer 1 tokens"
        );
    }
    #[test]
    fn can_punish_tests() {
        let mut account = Account::new("example");
        account.tokens = 10;
        assert!(
            account.can_punish(),
            "An account with 10 tokens should be able to be punished"
        );
        account.tokens = 1;
        assert!(
            account.can_punish(),
            "An account with 1 token should be able to be punished"
        );
        account.tokens = 0;
        assert!(
            !account.can_punish(),
            "An account with 0 tokens should not be able to be punished"
        );
    }
    #[test]
    fn miner_weight_tests() {
        let mut account = Account::new("example");
        account.tokens = 20;
        account.staked = 10;
        assert_eq!(
            account.weight_as_miner(),
            10,
            "An account with 20 tokens and 10 staked should have a weight of 10"
        );
        account.tokens = 10;
        assert_eq!(
            account.weight_as_miner(),
            10,
            "An account with 10 tokens and 10 staked should have a weight of 10"
        );
        account.staked = 0;
        assert_eq!(
            account.weight_as_miner(),
            0,
            "An account with 20 tokens and 0 staked should have a weight of 0"
        );
    }
    #[test]
    fn validator_weight_tests() {
        let mut account = Account::new("example");
        account.tokens = 20;
        account.staked = 10;
        assert_eq!(
            account.weight_as_validator(),
            10,
            "An account with 20 tokens and 10 staked should have a weight of 10"
        );
        account.tokens = 10;
        assert_eq!(
            account.weight_as_validator(),
            10,
            "An account with 10 tokens and 10 staked should have a weight of 10"
        );
        account.staked = 0;
        assert_eq!(
            account.weight_as_validator(),
            0,
            "An account with 20 tokens and 0 staked should have a weight of 0"
        );
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

    fn _fixture_accounts() -> (Account, Account, Account) {
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
