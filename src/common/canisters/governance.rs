use ic_agent::Agent;
use ic_utils::Canister;

const GOVERNANCE_CANISTER_ID: &str = "rrkah-fqaaa-aaaaa-aaaaq-cai";

pub fn canister(agent: &Agent) -> Canister {
    Canister::builder()
        .with_agent(agent)
        .with_canister_id(GOVERNANCE_CANISTER_ID)
        .build()
        .expect("Failed to governance create canister interface to communicate with the NNS.")
}
