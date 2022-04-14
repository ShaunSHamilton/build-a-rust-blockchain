extern crate blockchain;

use blockchain::{
    chain::{Chain, ChainTrait},
    mine_block,
    node::Node,
    Events, NodeState, Transaction,
};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use web_sys::{console, ErrorEvent};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn stake() {
    let mut fix_node_state = fix(None);
    fix_node_state.transactions[0].event = Events::Stake;
    fix_node_state.transactions[0].address = "Camper".to_string();

    let (chain, _) = mine(fix_node_state).expect("result to be chain");
    assert_eq!(chain.get_last_block().unwrap().data[0].staked, 1);
}

#[wasm_bindgen_test]
fn unstake() {
    let mut fix_node_state = fix(None);
    fix_node_state.chain[0].data[0].staked = 1;
    fix_node_state.transactions[0].event = Events::Unstake;
    let (chain, _) = mine(fix_node_state).expect("result to be chain");
    assert_eq!(chain.get_last_block().unwrap().data[0].staked, 0);
}

#[wasm_bindgen_test]
fn add_account() {
    let mut fix_node_state = fix(None);
    fix_node_state.transactions[0].event = Events::AddAccount;
    fix_node_state.transactions[0].address = "Camper".to_string();
    let (chain, _) = mine(fix_node_state).expect("result to be chain");
    let new_address = chain.get_last_block().unwrap().data[0].address.clone();
    assert!(chain.get_node_by_address(&new_address).is_some());
    assert_eq!(chain.get_nodes().len(), 3);
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
            "Invalid transactions. No change in chain"
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
            "Invalid transactions. No change in chain"
        );
    }
}

#[wasm_bindgen_test]
fn all_invalid_find_node() {
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
            "Invalid transactions. No change in chain"
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
    assert_eq!(chain.get_node_by_address("Camper").unwrap().staked, 2);
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

    assert_eq!(chain.get_node_by_address("Camper").unwrap().staked, 1);
    assert_eq!(errors[0], "'Ahmad' not found in chain");
}

fn fix(data: Option<Node>) -> NodeState {
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
