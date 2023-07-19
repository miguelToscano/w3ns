use candid::{CandidType, Principal};
use ic_kit::candid::{candid_method, export_service};
use ic_kit::ic;
use ic_kit::macros::*;
use ic_kit::*;
use serde::Deserialize;

use crate::domain::api_keys::services as api_keys_service;
use crate::domain::api_keys::types::{ApiKey, EthApiKey};
use crate::domain::emails::services as emails_service;
use crate::domain::emails::types::{QueuedEmail, SendEmailInput};
use crate::domain::push::services as push_service;
use crate::domain::push::types::{SendPushInput, SendPushToTopicInput, QueuedPush};
use crate::domain::sms::services as sms_service;
use crate::domain::sms::types::{SendSmsInput, QueuedSms};
use crate::domain::topics::services as topics_service;
use crate::domain::topics::types::{SubscribeUserToTopicInput, Topic, UnsubscribeUserFromTopic};
use crate::errors::ApiError;
use crate::repositories::api_keys::ApiKeys;
use crate::repositories::emails_queue::EmailsQueue;
use crate::repositories::eth_api_keys::EthApiKeys;
use crate::repositories::topics::Topics;

const SEND_EMAIL_FEE: u64 = 4_000_000_000;
const SEND_SMS_FEE: u64 = 4_000_000_000;
const SEND_PUSH_FEE: u64 = 4_000_000_000;
const SEND_PUSH_TO_TOPIC_FEE: u64 = 4_000_000_000;

const ENQUEUE_EMAIL_FEE: u64 = 25_000_000;
const ENQUEUE_SMS_FEE: u64 = 25_000_000;
const ENQUEUE_PUSH_FEE: u64 = 25_000_000;

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
pub fn register_eth_key(caller: String, api_key: String) -> Result<(), ApiError> {
    let eth_api_key = EthApiKey {
        value: api_key,
        owner: caller,
        created_at: ic::time(),
    };
    api_keys_service::register_eth(&eth_api_key)
}

#[update]
#[candid_method(update)]
pub async fn send_email(input: SendEmailInput) -> Result<u64, ApiError> {
    let available_cycles = ic::msg_cycles_available();
    ic::print(format!("Cycles availabe: {}", available_cycles));

    if available_cycles < SEND_EMAIL_FEE {
        return Err(ApiError::InsufficientCyclesReceived(format!(
            "Required cycles: {}",
            SEND_EMAIL_FEE
        )));
    }
    let accepted_cycles = ic::msg_cycles_accept(SEND_EMAIL_FEE);
    ic::print(format!("Cycles accepted: {}", accepted_cycles));
    let start_cycles = ic::balance();
    let caller = ic::caller();
    let api_key = api_keys_service::validate_api_key(&caller)?;
    emails_service::send_courier_email(&api_key.value, &input).await?;
    let end_cycles = ic::balance();
    return Ok(start_cycles - end_cycles);
}

#[update]
#[candid_method(update)]
pub async fn send_sms(input: SendSmsInput) -> Result<u64, ApiError> {
    let available_cycles = ic::msg_cycles_available();
    ic::print(format!("Cycles availabe: {}", available_cycles));

    if available_cycles < SEND_SMS_FEE {
        return Err(ApiError::InsufficientCyclesReceived(format!(
            "Required cycles: {}",
            SEND_SMS_FEE
        )));
    }

    let accepted_cycles = ic::msg_cycles_accept(SEND_SMS_FEE);
    ic::print(format!("Cycles accepted: {}", accepted_cycles));
    let start_cycles = ic::balance();
    let caller = ic::caller();
    let api_key = api_keys_service::validate_api_key(&caller)?;
    sms_service::send_courier_sms(&api_key.value, &input).await?;
    let end_cycles = ic::balance();
    return Ok(start_cycles - end_cycles);
}

#[update]
#[candid_method(update)]
pub async fn send_push(input: SendPushInput) -> Result<u64, ApiError> {
    let available_cycles = ic::msg_cycles_available();
    ic::print(format!("Cycles availabe: {}", available_cycles));

    if available_cycles < SEND_PUSH_FEE {
        return Err(ApiError::InsufficientCyclesReceived(format!(
            "Required cycles: {}",
            SEND_PUSH_FEE
        )));
    }

    let accepted_cycles = ic::msg_cycles_accept(SEND_PUSH_FEE);
    ic::print(format!("Cycles accepted: {}", accepted_cycles));
    let start_cycles = ic::balance();
    let caller = ic::caller();
    let api_key = api_keys_service::validate_api_key(&caller)?;
    push_service::send_courier_push(&api_key.value, &input).await?;
    let end_cycles = ic::balance();
    return Ok(start_cycles - end_cycles);
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

#[query]
#[candid_method(query)]
pub fn cycles() -> u64 {
    ic::balance()
}

#[update]
#[candid_method(update)]
pub async fn send_push_to_topic(input: SendPushToTopicInput) -> Result<u64, ApiError> {
    let available_cycles = ic::msg_cycles_available();
    ic::print(format!("Cycles availabe: {}", available_cycles));

    if available_cycles < SEND_PUSH_TO_TOPIC_FEE {
        return Err(ApiError::InsufficientCyclesReceived(format!(
            "Required cycles: {}",
            SEND_PUSH_TO_TOPIC_FEE
        )));
    }

    let accepted_cycles = ic::msg_cycles_accept(SEND_PUSH_TO_TOPIC_FEE);
    ic::print(format!("Cycles accepted: {}", accepted_cycles));
    let start_cycles = ic::balance();
    let caller = ic::caller();
    let api_key = api_keys_service::validate_api_key(&caller)?;
    let topic = topics_service::get_topic(&caller, input.clone().topic)?;
    push_service::send_courier_topic_push(&api_key.value, &input, topic.subscribers).await?;
    let end_cycles = ic::balance();
    return Ok(start_cycles - end_cycles);
}

#[update]
#[candid_method(update)]
pub async fn subscribe_user_to_topic(input: SubscribeUserToTopicInput) -> Result<(), ApiError> {
    let caller = ic::caller();
    topics_service::subscribe_user_to_topic(&caller, &input)
}

#[update]
#[candid_method(update)]
pub async fn unsubscribe_user_from_topic(input: UnsubscribeUserFromTopic) -> Result<(), ApiError> {
    let caller = ic::caller();
    topics_service::unsubscribe_user_from_topic(&caller, &input)
}

#[update]
#[candid_method(update)]
pub fn remove_key() -> Result<(), ApiError> {
    let caller = ic::caller();
    api_keys_service::validate_api_key(&caller)?;
    api_keys_service::delete(&caller)
}

#[update]
#[candid_method(update)]
pub fn remove_eth_key(caller: String) -> Result<(), ApiError> {
    api_keys_service::validate_eth_api_key(&caller)?;
    api_keys_service::delete_eth(&caller)
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

#[query]
#[candid_method(query)]
pub fn get_queued_email_notifications() -> Vec<QueuedEmail> {
    let queued_emails = emails_service::get_queued_emails();
    return queued_emails;
}

#[query]
#[candid_method(query)]
pub fn get_queued_sms_notifications() -> Vec<QueuedSms> {
    let queued_sms = sms_service::get_queued_sms();
    return queued_sms;
}

#[update]
#[candid_method(update)]
pub fn enqueue_email_notification(input: SendEmailInput) -> Result<(), ApiError> {
    let caller = ic::caller();
    let api_key = api_keys_service::validate_api_key(&caller)?;
    let available_cycles = ic::msg_cycles_available();
    ic::print(format!("Cycles availabe: {}", available_cycles));

    if available_cycles < ENQUEUE_EMAIL_FEE {
        return Err(ApiError::InsufficientCyclesReceived(format!(
            "Required cycles: {}",
            ENQUEUE_EMAIL_FEE
        )));
    }
    ic::msg_cycles_accept(ENQUEUE_EMAIL_FEE);
    emails_service::queue_email(&api_key.value, &input)?;
    return Ok(());
}

#[update]
#[candid_method(update)]
pub fn enqueue_eth_email_notification(caller: String, input: SendEmailInput) -> Result<(), ApiError> {
    let api_key = api_keys_service::validate_eth_api_key(&caller)?;
    emails_service::queue_email(&api_key.value, &input)?;
    return Ok(());
}

#[update]
#[candid_method(update)]
pub fn enqueue_sms_notification(input: SendSmsInput) -> Result<(), ApiError> {
    let caller = ic::caller();
    let api_key = api_keys_service::validate_api_key(&caller)?;
    let available_cycles = ic::msg_cycles_available();
    ic::print(format!("Cycles availabe: {}", available_cycles));

    if available_cycles < ENQUEUE_SMS_FEE {
        return Err(ApiError::InsufficientCyclesReceived(format!(
            "Required cycles: {}",
            ENQUEUE_SMS_FEE
        )));
    }
    ic::msg_cycles_accept(ENQUEUE_SMS_FEE);
    sms_service::queue_sms(&api_key.value, &input)?;
    return Ok(());
}

#[update]
#[candid_method(update)]
pub fn enqueue_eth_sms_notification(caller: String, input: SendSmsInput) -> Result<(), ApiError> {
    let api_key = api_keys_service::validate_eth_api_key(&caller)?;
    sms_service::queue_sms(&api_key.value, &input)?;
    return Ok(());
}

#[update]
#[candid_method(update)]
pub fn dequeue_email_notifications() -> Vec<QueuedEmail> {
    let queued_emails = emails_service::dequeue_emails();
    return queued_emails;
}

#[update]
#[candid_method(update)]
pub fn dequeue_sms_notifications() -> Vec<QueuedSms> {
    let queued_sms = sms_service::dequeue_sms();
    return queued_sms;
}

#[update]
#[candid_method(update)]
pub fn enqueue_push_notification(input: SendPushInput) -> Result<(), ApiError> {
    let caller = ic::caller();
    let api_key = api_keys_service::validate_api_key(&caller)?;
    let available_cycles = ic::msg_cycles_available();
    ic::print(format!("Cycles availabe: {}", available_cycles));

    if available_cycles < ENQUEUE_PUSH_FEE {
        return Err(ApiError::InsufficientCyclesReceived(format!(
            "Required cycles: {}",
            ENQUEUE_PUSH_FEE
        )));
    }
    ic::msg_cycles_accept(ENQUEUE_PUSH_FEE);
    push_service::queue_push(&api_key.value, &input)?;
    return Ok(());
}

#[query]
#[candid_method(query)]
pub fn get_queued_push_notifications() -> Vec<QueuedPush> {
    let queued_push = push_service::get_queued_push();
    return queued_push;
}


#[update]
#[candid_method(update)]
pub fn dequeue_push_notifications() -> Vec<QueuedPush> {
    let queued_push = push_service::dequeue_push();
    return queued_push;
}

#[derive(CandidType, Deserialize, Clone)]
pub struct StableStorage {
    api_keys: Vec<(Principal, ApiKey)>,
    eth_api_keys: Vec<(String, EthApiKey)>,
    topics: Vec<(Principal, Vec<Topic>)>,
    emails_queue: Vec<QueuedEmail>,
}

#[pre_upgrade]
pub fn pre_upgrade() {
    let api_keys = ic::with_mut(|api_keys_repository: &mut ApiKeys| api_keys_repository.archive());
    let eth_api_keys = ic::with_mut(|eth_api_keys_repository: &mut EthApiKeys| eth_api_keys_repository.archive());
    let topics = ic::with_mut(|topics_repository: &mut Topics| topics_repository.archive());
    let emails_queue = ic::with_mut(|emails_queue: &mut EmailsQueue| emails_queue.archive());

    let stable_storage = StableStorage { api_keys, eth_api_keys, topics, emails_queue };

    match ic::stable_store((stable_storage,)) {
        Ok(_) => (),
        Err(candid_err) => {
            ic::trap(&format!(
                "An error occurred when saving to stable memory (pre_upgrade): {:?}",
                candid_err
            ));
        }
    };
}

#[post_upgrade]
pub fn post_upgrade() {
    if let Ok((stable_storage,)) = ic::stable_restore::<(StableStorage,)>() {
        ic::with_mut(|topics_repository: &mut Topics| {
            topics_repository.load(stable_storage.clone().topics)
        });
        ic::with_mut(|api_keys_repository: &mut ApiKeys| {
            api_keys_repository.load(stable_storage.clone().api_keys)
        });
        ic::with_mut(|eth_api_keys_repository: &mut EthApiKeys| {
            eth_api_keys_repository.load(stable_storage.clone().eth_api_keys)
        });
        ic::with_mut(|emails_queue: &mut EmailsQueue| {
            emails_queue.load(stable_storage.clone().emails_queue)
        });
    }
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
