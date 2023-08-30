use cairo_felt::Felt252;
use camino::Utf8PathBuf;
use cheatnet::conversions::felt_selector_from_name;
use cheatnet::rpc::call_contract;
use cheatnet::{
    cheatcodes::ContractArtifacts, conversions::felt_from_short_string, rpc::CallContractOutput,
    CheatnetState,
};
use starknet_api::core::ContractAddress;
use std::{collections::HashMap, str::FromStr};

use crate::common::scarb::{get_contracts_map, try_get_starknet_artifacts_path};

static TARGET_NAME: &str = "cheatnet_testing_contracts";

pub mod assertions;
pub mod scarb;
pub mod state;

pub fn recover_data(output: CallContractOutput) -> Vec<Felt252> {
    match output {
        CallContractOutput::Success { ret_data } => ret_data,
        CallContractOutput::Panic { panic_data } => panic_data,
        CallContractOutput::Error { msg } => panic!("Call failed with message: {msg}"),
    }
}

pub fn get_contracts() -> HashMap<String, ContractArtifacts> {
    let contracts_path = Utf8PathBuf::from_str("tests/contracts")
        .unwrap()
        .canonicalize_utf8()
        .unwrap();
    let artifacts_path = try_get_starknet_artifacts_path(&contracts_path, TARGET_NAME)
        .unwrap()
        .unwrap();
    get_contracts_map(&artifacts_path)
}

pub fn deploy_contract(
    state: &mut CheatnetState,
    contract_name: &str,
    calldata: &[Felt252],
) -> ContractAddress {
    let contract = felt_from_short_string(contract_name);
    let contracts = get_contracts();

    let class_hash = state.declare(&contract, &contracts).unwrap();
    state.deploy(&class_hash, calldata).unwrap()
}

pub fn call_contract_getter_by_name(
    state: &mut CheatnetState,
    contract_address: &ContractAddress,
    fn_name: &str,
) -> CallContractOutput {
    let selector = felt_selector_from_name(fn_name);
    let result = call_contract(contract_address, &selector, vec![].as_slice(), state).unwrap();

    result
}