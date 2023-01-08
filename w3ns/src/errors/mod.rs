use ic_kit::candid::CandidType;
use serde::*;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum ApiError {
    ApiKeyNotFound,
    ApiKeyAlreadyExists,
    InvalidApiKey,
    InternalError,
    TopicAlreadyExists,
    TopicNotFound,
    SubscriberNotFound,
}
