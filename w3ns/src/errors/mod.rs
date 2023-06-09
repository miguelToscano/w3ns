use ic_kit::candid::CandidType;
use serde::*;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum ApiError {
    ApiKeyNotFound,
    ApiKeyAlreadyExists,
    InvalidApiKey,
    InternalError,
    InterCanisterCallError(String),
    TopicAlreadyExists,
    TopicNotFound,
    SubscriberNotFound,
    InsufficientCyclesReceived(String),
}
