use ic_kit::candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Topic {
    pub name: String,
    pub owner: Principal,
    pub subscribers: Vec<String>,
    pub created_at: u64,
}
