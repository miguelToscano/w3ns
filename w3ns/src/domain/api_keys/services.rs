use candid::Principal;
use ic_kit::ic;

use crate::domain::api_keys::types::{ApiKey, EthApiKey};
use crate::errors::ApiError;
use crate::repositories::api_keys::ApiKeys;
use crate::repositories::eth_api_keys::EthApiKeys;

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

pub fn register_eth(api_key: &EthApiKey) -> Result<(), ApiError> {
    ic::with_mut(|eth_api_keys_repository: &mut EthApiKeys| {
        let existing_api_key = eth_api_keys_repository.get(&api_key.owner);

        if existing_api_key.is_some() {
            return Err(ApiError::ApiKeyAlreadyExists);
        }

        eth_api_keys_repository
            .create(api_key)
            .map_err(|_| ApiError::InternalError)
    })
}

pub fn delete(owner: &Principal) -> Result<(), ApiError> {
    ic::with_mut(|api_keys_repository: &mut ApiKeys| {
        api_keys_repository
            .get(owner)
            .ok_or(ApiError::ApiKeyNotFound)?;
        api_keys_repository
            .delete(owner)
            .map_err(|_| ApiError::InternalError)
    })
}

pub fn delete_eth(owner: &String) -> Result<(), ApiError> {
    ic::with_mut(|eth_api_keys_repository: &mut EthApiKeys| {
        eth_api_keys_repository
            .get(owner)
            .ok_or(ApiError::ApiKeyNotFound)?;
        eth_api_keys_repository
            .delete(owner)
            .map_err(|_| ApiError::InternalError)
    })
}

pub fn validate_api_key(owner: &Principal) -> Result<ApiKey, ApiError> {
    ic::with(|api_keys_repository: &ApiKeys| {
        api_keys_repository
            .get(owner)
            .ok_or(ApiError::ApiKeyNotFound)
    })
}

pub fn validate_eth_api_key(owner: &String) -> Result<EthApiKey, ApiError> {
    ic::with(|eth_api_keys_repository: &EthApiKeys| {
        eth_api_keys_repository
            .get(owner)
            .ok_or(ApiError::ApiKeyNotFound)
    })
}

pub fn get_all() -> Vec<ApiKey> {
    ic::with(|api_keys_repository: &ApiKeys| api_keys_repository.get_all())
}

pub fn get_all_eth() -> Vec<EthApiKey> {
    ic::with(|eth_api_keys_repository: &EthApiKeys| eth_api_keys_repository.get_all())
}

pub fn get(owner: &Principal) -> Option<ApiKey> {
    ic::with(|api_keys_repository: &ApiKeys| api_keys_repository.get(owner))
}

pub fn get_eth(owner: &String) -> Option<EthApiKey> {
    ic::with(|eth_api_keys_repository: &EthApiKeys| eth_api_keys_repository.get(owner))
}
