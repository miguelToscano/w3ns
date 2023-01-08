use candid::Principal;
use ic_kit::ic;

use crate::domain::api_keys::types::ApiKey;
use crate::errors::ApiError;
use crate::repositories::api_keys::ApiKeys;

pub fn register(api_key: &ApiKey) -> Result<(), ApiError> {
    ic::with_mut(|api_keys_repository: &mut ApiKeys| {
        let existing_api_key = api_keys_repository.get(&api_key.owner);

        if existing_api_key.is_some() {
            return Err(ApiError::ApiKeyAlreadyExists);
        }

        api_keys_repository
            .create(api_key)
            .map_err(|_| ApiError::InternalError)
    })
}

pub fn delete(owner: &Principal) -> Result<(), ApiError> {
    ic::with_mut(|api_keys_repository: &mut ApiKeys| {
        api_keys_repository
            .delete(owner)
            .map_err(|_| ApiError::InternalError)
    })
}

pub fn validate_api_key(owner: &Principal) -> Result<ApiKey, ApiError> {
    ic::with(|api_keys_repository: &ApiKeys| {
        api_keys_repository.get(owner).ok_or(ApiError::ApiKeyNotFound)
    })
}

pub fn get_all() -> Vec<ApiKey> {
    ic::with(|api_keys_repository: &ApiKeys| api_keys_repository.get_all())
}

pub fn get(owner: &Principal) -> Option<ApiKey> {
    ic::with(|api_keys_repository: &ApiKeys| api_keys_repository.get(owner))
}
