extern crate blockchain;

use blockchain::{
    account::Account,
    block::Block,
    chain::{Chain, ChainTrait},
    validate_block,
};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use web_sys::ErrorEvent;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn validate_block_on_genesis_returns_error() {
    let fix_chain = fix(None);
    let chain_res = validate(fix_chain);
    assert!(chain_res.is_err());
    if let Err(e) = chain_res {
        // Get the error message
        let error_message = ErrorEvent::from(e);
        assert_eq!(error_message.message(), "Chain is too short");
    }
}

#[wasm_bindgen_test]
fn validate_block_on_empty_chain_returns_error() {
    let mut fix_chain = fix(None);
    fix_chain.pop();
    let chain_res = validate(fix_chain);
    assert!(chain_res.is_err());
    if let Err(e) = chain_res {
        // Get the error message
        let error_message = ErrorEvent::from(e);
        assert_eq!(error_message.message(), "Chain is too short");
    }
}

#[wasm_bindgen_test]
fn validate_block_on_valid_chain_returns_true() {
    let mut fix_chain = fix(None);
    let data = vec![Account::new("Mrugesh".to_string())];
    let network = vec!["Tom".to_string(), "Camper".to_string()];
    fix_chain.mine_block(data, network);
    let chain_res = validate(fix_chain);
    assert!(chain_res.is_ok());
    assert!(
        chain_res.unwrap(),
        "Chain should be validated, and `true` returned"
    );
}

fn fix(data: Option<Block>) -> Chain {
    if let Some(data) = data {
        let block = serde_json::to_string(&data).unwrap();
        let fix_chain = format!(
            r#"[
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
			}},
      {}
		  ]"#,
            block
        );

        serde_json::from_str(&fix_chain).unwrap()
    } else {
        let fix_chain = format!(
            r#"[
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
		  ]"#
        );

        serde_json::from_str(&fix_chain).unwrap()
    }
}

fn validate(chain: Chain) -> Result<bool, JsValue> {
    let chain = JsValue::from_serde(&chain).unwrap();
    let res = validate_block(chain);
    match res {
        Ok(validity) => Ok(validity),
        Err(e) => {
            // Error is converted into a JsValue to make use of Debug trait
            Err(JsValue::from(e))
        }
    }
}
