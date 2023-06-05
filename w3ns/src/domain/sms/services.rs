use crate::errors::ApiError;
use crate:: domain::sms::types::{QueuedSms, SendSmsInput};
use crate::repositories::sms_queue::SmsQueue;
use candid::Principal;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext,
};
use ic_kit::ic;
use uuid::Uuid;

pub async fn send_courier_sms(api_key: &str, sms: &SendSmsInput) -> Result<(), ApiError> {
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
        body: Some(sms.to_courier_format().into_bytes()),
        max_response_bytes: Some(1024),
        transform: Some(TransformContext::new(transform_send_sms, vec![])),
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
pub fn transform_send_sms(raw: TransformArgs) -> HttpResponse {
    let mut sanitized = raw.response;
    sanitized.headers = vec![];
    sanitized
}

pub fn queue_sms(api_key: &str, sms: &SendSmsInput) -> Result<(), ApiError> {
    ic::with_mut(|sms_queue: &mut SmsQueue| {
        let sms = QueuedSms {
            api_key: api_key.to_owned(),
            to: sms.to.clone(),
            message: sms.message.clone(),
        };

        sms_queue.enqueue(sms);
    });

    return Ok(());
}

pub fn get_queued_sms() -> Vec<QueuedSms> {
    ic::with(|sms_queue: &SmsQueue| {
        let queued_sms = sms_queue.get_all();
        queued_sms
    })
}

pub fn dequeue_sms() -> Vec<QueuedSms> {
    ic::with_mut(|sms_queue: &mut SmsQueue| {
        sms_queue.dequeue_all()
    })
}
