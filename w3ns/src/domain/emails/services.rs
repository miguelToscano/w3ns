use crate::{errors::ApiError, repositories::emails_queue::EmailsQueue};
use candid::Principal;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext,
};
use ic_kit::ic;
use uuid::Uuid;

use crate::domain::emails::types::SendEmailInput;

use super::types::QueuedEmail;

pub async fn send_courier_email(api_key: &str, email: &SendEmailInput) -> Result<(), ApiError> {
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
        body: Some(email.to_courier_format().into_bytes()),
        max_response_bytes: Some(1024),
        transform: Some(TransformContext::new(transform_send_email, vec![])),
        headers: request_headers,
    };

    match http_request(request).await {
        Ok((_response,)) => {
            ic::print(format!("{:?}", _response.body.len()));
            Ok(())
        }
        Err((_r, _m)) => {
            ic::print(format!("{:?} ------ {:?}", _r, _m));
            Err(ApiError::InterCanisterCallError(_m))
        }
    }
}

#[ic_cdk_macros::query]
pub fn transform_send_email(raw: TransformArgs) -> HttpResponse {
    let mut sanitized = raw.response;
    sanitized.headers = vec![];
    sanitized
}

pub fn queue_email(api_key: &str, email_input: &SendEmailInput) -> Result<(), ApiError> {
    ic::with_mut(|emails_queue: &mut EmailsQueue| {
        let email = QueuedEmail {
            api_key: api_key.to_owned(),
            to: email_input.to.clone(),
            title: email_input.title.clone(),
            body: email_input.body.clone(),
        };

        emails_queue.enqueue(email);
    });

    return Ok(());
}

pub fn get_queued_emails() -> Vec<QueuedEmail> {
    ic::with(|emails_queue: &EmailsQueue| emails_queue.get_all())
}

pub fn dequeue_emails() -> Vec<QueuedEmail> {
    ic::with_mut(|emails_queue: &mut EmailsQueue| emails_queue.dequeue_all())
}
