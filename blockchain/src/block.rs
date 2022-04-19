//! # Block
//!
//! A block is a piece of data that is stored in the blockchain.

/// TODO: Complete the struct definition.
/// The block added to the chain of the blockchain.
///
/// **Note:** This is a reference type, and does not contain any implementations.
pub struct Block {}

// DO NOT EDIT TESTS
#[cfg(test)]
mod tests {
    use super::*;
    use crate::account::{Account, AccountTrait};
    #[test]
    fn block_has_correct_fields() {
        // This test must just compile.
        let _block = Block {
            id: 0u64,
            hash: "example".to_string(),
            previous_hash: "example".to_string(),
            timestamp: 0u64,
            data: vec![],
            nonce: 0u64,
            next_miner: "Example".to_string(),
            next_validators: vec![String::from("Example")],
        };
    }
    #[test]
    fn block_data_is_vec_of_accounts() {
        // This test must just compile.
        let _block = Block {
            id: 0u64,
            hash: "example".to_string(),
            previous_hash: "example".to_string(),
            timestamp: 0u64,
            data: vec![Account::new("example")],
            nonce: 0u64,
            next_miner: "Example".to_string(),
            next_validators: vec![String::from("Example")],
        };
    }
}
