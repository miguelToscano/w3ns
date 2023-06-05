use crate::errors::ApiError;
use candid::Principal;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext,
};
use ic_kit::ic;
use uuid::Uuid;

use crate::domain::push::types::{SendPushInput, SendPushToTopicInput, QueuedPush};
use crate::repositories::push_queue::PushQueue;

pub async fn send_courier_push(
    api_key: &str,
    push_notification: &SendPushInput,
) -> Result<(), ApiError> {
    let (bytes,): (Vec<u8>,) = ic::call(Principal::management_canister(), "raw_rand", ())
        .await
        .map_err(|(_, _)| ApiError::InternalError)?;

    let idempotency_key = Uuid::from_slice(&(bytes)[..16])
        .map_err(|_| ApiError::InternalError)?
        .to_string();

    let request_headers: Vec<HttpHeader> = vec![
        HttpHeader {
            name: "Authorization".to_owned(),
            value: format!("Bearer {}", api_key),
        },
        HttpHeader {
            name: "Idempotency-Key".to_owned(),
            value: idempotency_key,
        },
        HttpHeader {
            name: "content-type".to_owned(),
            value: "application/json".to_owned(),
        },
    ];

    let request = CanisterHttpRequestArgument {
        url: "https://us-central1-courier-api-proxy.cloudfunctions.net/send".to_string(),
        method: HttpMethod::POST,
        body: Some(push_notification.to_courier_format().into_bytes()),
        max_response_bytes: Some(1024),
        transform: Some(TransformContext::new(transform_send_push, vec![])),
        headers: request_headers,
    };

    match http_request(request).await {
        Ok((_response,)) => Ok(()),
        Err((_r, _m)) => {
            ic::print(format!("{:?} ------ {:?}", _r, _m));
            Err(ApiError::InterCanisterCallError(_m))
        }
    }
}

pub async fn send_courier_topic_push(
    api_key: &str,
    push_notification: &SendPushToTopicInput,
    subscribers: Vec<String>,
) -> Result<(), ApiError> {
    let (bytes,): (Vec<u8>,) = ic::call(Principal::management_canister(), "raw_rand", ())
        .await
        .map_err(|(_, _)| ApiError::InternalError)?;

    let idempotency_key = Uuid::from_slice(&(bytes)[..16])
        .map_err(|_| ApiError::InternalError)?
        .to_string();

    let request_headers: Vec<HttpHeader> = vec![
        HttpHeader {
            name: "Authorization".to_owned(),
            value: format!("Bearer {}", api_key),
        },
        HttpHeader {
            name: "Idempotency-Key".to_owned(),
            value: idempotency_key,
        },
        HttpHeader {
            name: "content-type".to_owned(),
            value: "application/json".to_owned(),
        },
    ];

    let request = CanisterHttpRequestArgument {
        url: "https://us-central1-courier-api-proxy.cloudfunctions.net/send".to_string(),
        method: HttpMethod::POST,
        body: Some(
            push_notification
                .to_courier_format(subscribers)
                .into_bytes(),
        ),
        max_response_bytes: Some(1024),
        transform: Some(TransformContext::new(transform_send_push, vec![])),
        headers: request_headers,
    };

    match http_request(request).await {
        Ok((_response,)) => Ok(()),
        Err((_r, _m)) => {
            ic::print(format!("{:?} ------ {:?}", _r, _m));
            Err(ApiError::InterCanisterCallError(_m))
        }
    }
}

#[ic_cdk_macros::query]
pub fn transform_send_push(raw: TransformArgs) -> HttpResponse {
    let mut sanitized = raw.response;
    sanitized.headers = vec![];
    sanitized
}

pub fn queue_push(api_key: &str, push_notification: &SendPushInput) -> Result<(), ApiError> {
    ic::with_mut(|push_queue: &mut PushQueue| {
        let queued_push = QueuedPush {
            api_key: api_key.to_owned(),
            firebase_token: push_notification.firebase_token.to_owned(),
            title: push_notification.title.to_owned(),
            body: push_notification.body.to_owned(),
        };

        push_queue.enqueue(queued_push);
    });

    return Ok(());
}

pub fn get_queued_push() -> Vec<QueuedPush> {
    ic::with(|push_queue: &PushQueue| push_queue.get_all())
}

pub fn dequeue_push() -> Vec<QueuedPush> {
    ic::with_mut(|push_queue: &mut PushQueue| push_queue.dequeue_all())
}
