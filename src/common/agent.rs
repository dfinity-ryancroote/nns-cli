use ic_agent::{agent::http_transport::ReqwestHttpReplicaV2Transport, Agent};

const MAINNET: &str = "https://ic0.app";

pub fn anonymous_agent() -> Agent {
    let transport = ReqwestHttpReplicaV2Transport::create(MAINNET)
        .expect("Failed to create a transport for to communicate with the Internet Computer.");
    Agent::builder()
        .with_transport(transport)
        .build()
        .expect("Failed to create an agent to communicate with the Internet Computer.")
}
