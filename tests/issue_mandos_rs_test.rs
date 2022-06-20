use std::fs;
use elrond_wasm_debug::BlockchainMock;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();

    blockchain.register_contract_builder("file:output/issue.wasm", issue::ContractBuilder);
    blockchain
}

#[test]
fn init_rs() {
    elrond_wasm_debug::mandos_rs("mandos/init.scen.json", world());
}

#[test]
fn problematic_endpoint_rs() {
    elrond_wasm_debug::mandos_rs("mandos/problematic_endpoint.scen.json", world());
}