//! # Block
//!
//! A block is a piece of data that is stored in the blockchain.

use serde::{Deserialize, Serialize};

use crate::node::Node;

/// The block added to the chain of the blockchain.
///
/// **Note:** This is a reference type, and does not contain any implementations.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: u64, // TODO: Consider removing if testing too difficult
    pub data: Vec<Node>,
    pub nonce: u64,
    pub next_miner: String,
    pub next_validators: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn block_has_correct_fields() {
        // This test must just not panic.
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
}
