use candid::Principal;
use ic_kit::ic;

use crate::domain::api_keys::types::ApiKey;
use crate::repositories::api_keys::ApiKeys;

pub fn create(api_key: &ApiKey) -> Result<(), ()> {
    ic::with_mut(|api_keys_repository: &mut ApiKeys| {
        api_keys_repository.create(api_key)
    })
}

pub fn delete(owner: &Principal) -> Result<(), ()> {
    ic::with_mut(|api_keys_repository: &mut ApiKeys| {
        api_keys_repository.delete(owner)
    })
}

pub fn get_all() -> Vec<ApiKey> {
    ic::with(|api_keys_repository: &ApiKeys| {
        api_keys_repository.get_all()
    })
}

pub fn get(owner: &Principal) -> Option<ApiKey> {
    ic::with(|api_keys_repository: &ApiKeys| {
        api_keys_repository.get(owner)
    })
}