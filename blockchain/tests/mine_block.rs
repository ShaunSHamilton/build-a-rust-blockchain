// DO NOT EDIT THIS FILE
extern crate blockchain;

use blockchain::{
    account::Account,
    chain::{Chain, ChainTrait},
    mine_block, Events, NodeState, Transaction,
};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use web_sys::{console, ErrorEvent};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn staking_increases_account_staked_by_1() {
    let mut fix_node_state = fix(None);
    fix_node_state.transactions[0].event = Events::Stake;

    let (chain, _) = mine(fix_node_state).expect("result to be chain");
    assert_eq!(
        chain.get_account_by_address("Camper").unwrap().staked,
        1,
        "Stake should increase by 1"
    );
    assert_eq!(
        chain.get_account_by_address("Camper").unwrap().tokens,
        20,
        "Tokens should not change"
    );
}

#[wasm_bindgen_test]
fn staking_with_no_tokens_returns_error() {
    let mut fix_node_state = fix(None);
    fix_node_state.chain[0].data[0].tokens = 0;
    fix_node_state.transactions[0].event = Events::Stake;
    fix_node_state.transactions.push(Transaction {
        event: Events::Stake,
        address: "Tom".to_string(),
    });

    let (chain, errors) = mine(fix_node_state).expect("result to be chain");

    assert_eq!(
        chain.get_account_by_address("Camper").unwrap().staked,
        0,
        "Stake should not change, if the transaction fails"
    );
    assert_eq!(
        chain.get_account_by_address("Camper").unwrap().tokens,
        0,
        "Tokens should not change"
    );
    assert_eq!(
        errors[0], "'Camper' cannot stake",
        "The expected error should be returned describing the failed transaction"
    );
}

#[wasm_bindgen_test]
fn staking_when_staked_equals_tokens_returns_error() {
    let mut fix_node_state = fix(None);
    fix_node_state.chain[0].data[0].tokens = 19;
    fix_node_state.chain[0].data[0].staked = 19;
    fix_node_state.transactions[0].event = Events::Stake;
    fix_node_state.transactions.push(Transaction {
        event: Events::Stake,
        address: "Tom".to_string(),
    });

    let (chain, errors) = mine(fix_node_state).expect("result to be chain");

    assert_eq!(
        chain.get_account_by_address("Camper").unwrap().staked,
        19,
        "Stake should not change, if the transaction fails"
    );
    assert_eq!(
        chain.get_account_by_address("Camper").unwrap().tokens,
        19,
        "Tokens should not change"
    );
    assert_eq!(
        errors[0], "'Camper' cannot stake",
        "The expected error should be returned describing the failed transaction"
    );
}

#[wasm_bindgen_test]
fn unstaking_decreases_account_staked_by_1() {
    let mut fix_node_state = fix(None);
    fix_node_state.chain[0].data[0].staked = 1;
    fix_node_state.transactions[0].event = Events::Unstake;
    let (chain, _) = mine(fix_node_state).expect("result to be chain");
    assert_eq!(
        chain.get_account_by_address("Camper").unwrap().staked,
        0,
        "Stake should decrease by 1"
    );
    assert_eq!(
        chain.get_account_by_address("Camper").unwrap().tokens,
        20,
        "Tokens should not change"
    );
}

#[wasm_bindgen_test]
fn add_account_adds_account() {
    let mut fix_node_state = fix(None);
    fix_node_state.transactions[0].event = Events::AddAccount;
    let ahmad = String::from("Ahmad");
    fix_node_state.transactions[0].address = ahmad.clone();
    let (chain, _) = mine(fix_node_state).expect("result to be chain");
    assert_eq!(
        chain.get_accounts().len(),
        3,
        "One more account should be added"
    );
    assert!(
        chain.get_account_by_address(&ahmad).is_some(),
        "New account should have correct address"
    );
}

#[wasm_bindgen_test]
fn all_invalid_unstake() {
    let mut fix_node_state = fix(None);
    fix_node_state.transactions[0].event = Events::Unstake;
    fix_node_state.transactions[0].address = "Camper".to_string();
    let chain_res = mine(fix_node_state);
    assert!(chain_res.is_err());
    if let Err(e) = chain_res {
        // Get the error message
        let error_message = ErrorEvent::from(e);
        assert_eq!(
            error_message.message(),
            "Invalid transactions. No change in chain",
            "The expected error should be returned describing the failed event"
        );
    }
}

#[wasm_bindgen_test]
fn all_invalid_stake() {
    let mut fix_node_state = fix(None);
    fix_node_state.chain[0].data[0].staked = 20;
    fix_node_state.transactions[0].event = Events::Stake;
    fix_node_state.transactions[0].address = "Camper".to_string();
    let chain_res = mine(fix_node_state);
    assert!(chain_res.is_err());
    if let Err(e) = chain_res {
        // Get the error message
        let error_message = ErrorEvent::from(e);
        assert_eq!(
            error_message.message(),
            "Invalid transactions. No change in chain",
            "The expected error should be returned describing the failed event"
        );
    }
}

#[wasm_bindgen_test]
fn all_invalid_find_account() {
    let mut fix_node_state = fix(None);
    fix_node_state.transactions[0].event = Events::Stake;
    fix_node_state.transactions[0].address = "Test".to_string();
    let chain_res = mine(fix_node_state);
    assert!(chain_res.is_err());
    if let Err(e) = chain_res {
        // Get the error message
        let error_message = ErrorEvent::from(e);
        assert_eq!(
            error_message.message(),
            "Invalid transactions. No change in chain",
            "The expected error should be returned describing the failed event"
        );
    }
}

#[wasm_bindgen_test]
fn stake_multiple_tokens() {
    let mut fix_node_state = fix(None);
    fix_node_state.transactions.push(Transaction {
        address: "Camper".to_string(),
        event: Events::Stake,
    });
    fix_node_state.transactions.push(Transaction {
        address: "Camper".to_string(),
        event: Events::Stake,
    });
    let (chain, _) = mine(fix_node_state).expect("result to be chain");
    assert_eq!(
        chain.get_account_by_address("Camper").unwrap().staked,
        2,
        "Two `Stake` events should increase the staked amount by 2"
    );
}

#[wasm_bindgen_test]
fn one_invalid_transaction() {
    let mut fix_node_state = fix(None);
    fix_node_state.transactions.push(Transaction {
        address: "Camper".to_string(),
        event: Events::Stake,
    });
    fix_node_state.transactions.push(Transaction {
        address: "Ahmad".to_string(),
        event: Events::Stake,
    });
    let (chain, errors) = mine(fix_node_state).expect("result to be chain");

    assert_eq!(
        chain.get_account_by_address("Camper").unwrap().staked,
        1,
        "One `Stake` event should increase the staked amount by 1"
    );
    assert_eq!(
        errors[0], "'Ahmad' not found in chain",
        "The expected error should be returned describing one failed event"
    );
}

fn fix(data: Option<Account>) -> NodeState {
    if let Some(data) = data {
        let node_vec_str = serde_json::to_string(&data).unwrap();
        let fix_node_state = format!(
            r#"{{
		  "chain": [
			{{
			  "id": 0,
			  "hash": "00110101",
			  "previous_hash": "",
			  "timestamp": 123456789,
			  "data": [{{ "address": "Camper", "staked": 0, "tokens": 20 }},
			  {{ "address": "Tom", "staked": 20, "tokens": 100 }}, {}],
			  "nonce": 123,
			  "next_miner": "Camper",
			  "next_validators": ["Tom"]
			}}
		  ],
		  "transactions": [
			{{
			  "address": "Camper",
			  "event": {{"Transfer": ["Tom", 1]}}
			}}
		  ],
		  "network": ["Camper"]
		}}"#,
            node_vec_str
        );

        serde_json::from_str(&fix_node_state).unwrap()
    } else {
        let fix_node_state = format!(
            r#"{{
	  "chain": [
		{{
		  "id": 0,
		  "hash": "00110101",
		  "previous_hash": "",
		  "timestamp": 123456789,
		  "data": [{{ "address": "Camper", "staked": 0, "tokens": 20 }},
		  {{ "address": "Tom", "staked": 20, "tokens": 100 }}],
		  "nonce": 123,
		  "next_miner": "Camper",
		  "next_validators": ["Tom"]
		}}
	  ],
	  "transactions": [
		{{
		  "address": "Camper",
		  "event": {{"Transfer": ["Tom", 1]}}
		}}
	  ],
	  "network": ["Camper"]
	}}"#
        );
        serde_json::from_str(&fix_node_state).unwrap()
    }
}

fn mine(fix_node_state: NodeState) -> Result<(Chain, Vec<String>), JsValue> {
    let node_state = JsValue::from_serde(&fix_node_state).unwrap();
    let res = mine_block(node_state);
    let response = match res {
        Ok(v) => match v.into_serde() {
            Ok(v) => v,
            Err(e) => {
                console::log_1(&format!("{:?}", e).into());
                panic!("could not serde response");
            }
        },
        Err(e) => {
            // Error is converted into a JsValue to make use of Debug trait
            return Err(JsValue::from(e));
        }
    };
    Ok(response)
}
