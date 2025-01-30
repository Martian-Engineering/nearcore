use borsh::{BorshDeserialize, BorshSerialize};
use near_schema_checker_lib::ProtocolSchema;

/// A simple request from one node to another
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Clone, Debug, ProtocolSchema)]
pub struct PoRRequest {
    pub payload: Vec<u8>,
}

/// A simple response from the target node 
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Clone, Debug, ProtocolSchema)]
pub struct PoRResponse {
    pub payload: Vec<u8>,
}