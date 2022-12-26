use candid::Principal;
use ic_kit::candid::{candid_method, export_service};
use ic_kit::macros::*;
use ic_kit::ic;

use crate::domain::api_keys::types::{ApiKey};
use crate::domain::api_keys::services as api_keys_service;
// use ic_kit::*;

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
            value: key.clone(),
            owner: caller.clone(),
            created_at: ic::time(),
        };

    api_keys_service::create(&api_key)
}

#[update]
#[candid_method(update)]
pub fn remove_key() -> Result<(), ()> {
    let caller = ic::caller();
    return Ok(());
}

#[query]
#[candid_method(query)]
pub fn has_key_registered() -> bool {
    let caller = ic::caller();
    return true;
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