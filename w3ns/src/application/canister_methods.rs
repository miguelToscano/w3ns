use candid::Principal;
use ic_kit::candid::{candid_method, export_service};
use ic_kit::ic;
use ic_kit::macros::*;

use crate::domain::api_keys::services as api_keys_service;
use crate::domain::api_keys::types::ApiKey;
use crate::domain::emails::services as emails_service;
use crate::domain::emails::types::Email;
use crate::domain::sms::services as sms_service;
use crate::domain::sms::types::Sms;
use crate::errors::ApiError;
use ic_kit::*;

#[query]
#[candid_method(query)]
pub fn name() -> String {
    String::from("W3NS")
}

#[update]
#[candid_method(update)]
pub fn register_key(key: String) -> Result<(), ()> {
    let caller = ic::caller();
    let api_key = ApiKey {
        value: key,
        owner: caller,
        created_at: ic::time(),
    };
    api_keys_service::create(&api_key)
}

#[update]
#[candid_method(update)]
pub async fn send_email(to: String, subject: String, body: String) -> Result<(), ApiError> {
    let caller = ic::caller();
    let api_key = api_keys_service::get(&caller).ok_or(ApiError::ApiKeyNotFound)?;
    let email = Email { to, subject, body };
    emails_service::send_courier_email(&api_key.value, &email).await
}

#[update]
#[candid_method(update)]
pub async fn send_sms(to: String, message: String) -> Result<(), ApiError> {
    let caller = ic::caller();
    let api_key = api_keys_service::get(&caller).ok_or(ApiError::ApiKeyNotFound)?;
    let sms = Sms { to, message };
    sms_service::send_courier_sms(&api_key.value, &sms).await
}

#[update]
#[candid_method(update)]
pub fn remove_key() -> Result<(), ApiError> {
    let caller = ic::caller();
    api_keys_service::get(&caller).ok_or(ApiError::ApiKeyNotFound)?;
    api_keys_service::delete(&caller).map_err(|_| ApiError::InternalError)
}

#[query]
#[candid_method(query)]
pub fn has_key_registered() -> bool {
    let caller = ic::caller();
    api_keys_service::get(&caller).is_some()
}

#[query]
#[candid_method(query)]
pub fn whoami() -> Principal {
    ic::caller()
}

#[query]
#[candid_method(query)]
pub fn get_all() -> Vec<ApiKey> {
    api_keys_service::get_all()
}

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        println!("{:?}", dir);
        let dir = dir.parent().unwrap().join("candid");
        write(dir.join("w3ns.did"), export_candid()).expect("Write failed.");
    }
}
