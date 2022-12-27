use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::http_request::{
    self, http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse,
    TransformArgs, TransformContext,
};
use ic_kit::candid::{candid_method, export_service};
use ic_kit::ic;
use ic_kit::macros::*;
use uuid::Uuid;

use crate::domain::api_keys::services as api_keys_service;
use crate::domain::api_keys::types::ApiKey;
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
        value: key.clone(),
        owner: caller.clone(),
        created_at: ic::time(),
    };

    api_keys_service::create(&api_key)
}

#[update]
#[candid_method(update)]
pub async fn send_email() -> Result<(), ApiError> {
    let caller = ic::caller();

    let api_key = api_keys_service::get(&caller).unwrap();

    let (bytes,): (Vec<u8>,) = ic::call(Principal::management_canister(), "raw_rand", ())
        .await
        .map_err(|(_, _)| ApiError::InternalError)?;

    let idempotency_key = Uuid::from_slice(&(bytes)[..16])
        .map_err(|_| ApiError::InternalError)?
        .to_string();

    ic::print("Despues de armar la idempotency key");

    let host = String::from("https://api.courier.com/send");

    let request_headers: Vec<HttpHeader> = vec![
        HttpHeader {
            name: "Authorization".to_owned(),
            value: format!("Bearer {}", api_key.value.clone()),
        },
        HttpHeader {
            name: "Idempotency-Key".to_owned(),
            value: idempotency_key,
        },
    ];

    let body = String::from(
        "{
      \"message\": {
        \"to\": {\"email\":\"miguetoscano288@gmail.com\"},
        \"content\": {
          \"title\": \"Welcome to Courier!\",
          \"body\": \"Want to hear a joke? {{joke}}\"
        },
        \"data\": {\"joke\": \"How did the T-Rex feel after a set of bicep curls? Dino-sore!\"}
        }
    }",
    );

    ic::print("Antes de armar el argument");

    let test = body.clone().into_bytes();
    ic::print("hace bien el into_bytes()");

    let request = CanisterHttpRequestArgument {
        url: host,
        method: HttpMethod::POST,
        body: Some(body.clone().into_bytes()),
        max_response_bytes: None,
        transform: Some(TransformContext::new(transform, vec![])),
        headers: request_headers,
    };

    ic::print(format!("{:?}", request));

    match http_request(request).await {
        Ok((response,)) => {
            println!("Ok")
        }
        Err((r, m)) => {
            println!("Error")
        }
    };

    Ok(())
}

#[ic_cdk_macros::query]
pub fn transform(raw: TransformArgs) -> HttpResponse {
    let mut sanitized = raw.response.clone();
    sanitized.headers = vec![];
    sanitized
}

#[update]
#[candid_method(update)]
pub fn remove_key() -> Result<(), ()> {
    let caller = ic::caller();
    api_keys_service::delete(&caller)
}

#[query]
#[candid_method(query)]
pub fn has_key_registered() -> bool {
    let caller = ic::caller();
    let api_key = api_keys_service::get(&caller);
    api_key.is_some()
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
