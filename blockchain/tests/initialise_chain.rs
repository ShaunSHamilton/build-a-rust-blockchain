// DO NOT EDIT THIS FILE
extern crate blockchain;

use blockchain::{
    account::{Account, AccountTrait},
    chain::{Chain, ChainTrait},
    initialise_chain,
};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use web_sys::console;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn initialise_chain_returns_chain() {
    let chain = init("Camper".to_string());
    assert!(chain.is_ok());
    let mut chain = chain.expect("no errors when initialising chain");

    // Create and mine genesis block
    let genesis_node = Account::new("Camper");
    let genesis_address = genesis_node.address.clone();
    let data = vec![genesis_node];
    let network = vec![genesis_address];
    chain.mine_block(data, network);
    assert!(chain.get_account_by_address("Camper").is_some());
}

fn init(address: String) -> Result<Chain, JsValue> {
    let res = initialise_chain(address);
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
