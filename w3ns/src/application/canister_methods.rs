use candid::Principal;
use ic_kit::candid::{candid_method, export_service};
use ic_kit::ic;
use ic_kit::macros::*;
use ic_kit::*;

use crate::domain::api_keys::services as api_keys_service;
use crate::domain::api_keys::types::ApiKey;
use crate::domain::emails::services as emails_service;
use crate::domain::emails::types::Email;
use crate::domain::push::services as push_service;
use crate::domain::push::types::{MultiplePush, Push};
use crate::domain::sms::services as sms_service;
use crate::domain::sms::types::Sms;
use crate::domain::topics::services as topics_service;
use crate::domain::topics::types::Topic;
use crate::errors::ApiError;

#[query]
#[candid_method(query)]
pub fn name() -> String {
    String::from("W3NS")
}

#[update]
#[candid_method(update)]
pub fn register_key(key: String) -> Result<(), ApiError> {
    let caller = ic::caller();
    let api_key = ApiKey {
        value: key,
        owner: caller,
        created_at: ic::time(),
    };
    api_keys_service::register(&api_key)
}

#[update]
#[candid_method(update)]
pub async fn send_email(to: String, subject: String, body: String) -> Result<(), ApiError> {
    let caller = ic::caller();
    let api_key = api_keys_service::validate_api_key(&caller)?;
    let email = Email { to, subject, body };
    emails_service::send_courier_email(&api_key.value, &email).await
}

#[update]
#[candid_method(update)]
pub async fn send_sms(to: String, message: String) -> Result<(), ApiError> {
    let caller = ic::caller();
    let api_key = api_keys_service::validate_api_key(&caller)?;
    let sms = Sms { to, message };
    sms_service::send_courier_sms(&api_key.value, &sms).await
}

#[update]
#[candid_method(update)]
pub async fn send_push_notification(
    firebase_token: String,
    title: String,
    body: String,
) -> Result<(), ApiError> {
    let caller = ic::caller();
    let api_key = api_keys_service::validate_api_key(&caller)?;
    let push_notification = Push {
        firebase_token,
        title,
        body,
    };
    push_service::send_courier_push(&api_key.value, &push_notification).await
}

#[query]
#[candid_method(query)]
pub fn get_topics() -> Vec<Topic> {
    let caller = ic::caller();
    topics_service::get_topics(&caller)
}

#[update]
#[candid_method(update)]
pub fn create_topic(topic_name: String) -> Result<(), ApiError> {
    let caller = ic::caller();
    let topic = Topic {
        name: topic_name,
        owner: caller.clone(),
        subscribers: vec![],
        created_at: ic::time(),
    };
    topics_service::create_topic(&caller, &topic)
}

#[update]
#[candid_method(update)]
pub fn delete_topic(topic_name: String) -> Result<(), ApiError> {
    let caller = ic::caller();
    topics_service::delete_topic(&caller, topic_name)
}

#[update]
#[candid_method(update)]
pub async fn send_push_to_topic(
    topic: String,
    title: String,
    body: String,
) -> Result<(), ApiError> {
    let caller = ic::caller();
    let api_key = api_keys_service::validate_api_key(&caller)?;
    let topic = topics_service::get_topic(&caller, topic)?;
    let push_notifications = MultiplePush {
        firebase_tokens: topic.subscribers.clone(),
        title,
        body,
    };
    push_service::send_courier_push(&api_key.value, &push_notifications).await
}

#[update]
#[candid_method(update)]
pub async fn subscribe_user_to_topic(
    registration_token: String,
    topic: String,
) -> Result<(), ApiError> {
    let caller = ic::caller();
    topics_service::subscribe_user_to_topic(&caller, topic, registration_token)
}

#[update]
#[candid_method(update)]
pub async fn unsubscribe_user_from_topic(
    registration_token: String,
    topic: String,
) -> Result<(), ApiError> {
    let caller = ic::caller();
    topics_service::unsubscribe_user_from_topic(&caller, topic, registration_token)
}

#[update]
#[candid_method(update)]
pub fn remove_key() -> Result<(), ApiError> {
    let caller = ic::caller();
    api_keys_service::validate_api_key(&caller)?;
    api_keys_service::delete(&caller)
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
