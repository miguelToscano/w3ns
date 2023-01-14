use ic_kit::{candid::CandidType, ic};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Fee {
    pub operation: String,
    pub value: u64,
}