use ic_kit::candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ApiKey {
    pub value: String,
    pub owner: Principal,
    pub created_at: u64,
}