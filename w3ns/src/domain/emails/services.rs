use crate::errors::ApiError;
use candid::Principal;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext,
};
use ic_kit::ic;
use uuid::Uuid;

const COURIER_SEND_EMAIL_URL: &str = "https://api.courier.com/send";

pub async fn send_courier_email(api_key: &str, email: &str) -> Result<(), ApiError> {
    let (bytes,): (Vec<u8>,) = ic::call(Principal::management_canister(), "raw_rand", ())
        .await
        .map_err(|(_, _)| ApiError::InternalError)?;

    let idempotency_key = Uuid::from_slice(&(bytes)[..16])
        .map_err(|_| ApiError::InternalError)?
        .to_string();

    ic::print("Despues de armar la idempotency key");

    let host = String::from(COURIER_SEND_EMAIL_URL);

    let request_headers: Vec<HttpHeader> = vec![
        HttpHeader {
            name: "Authorization".to_owned(),
            value: format!("Bearer {}", api_key.clone()),
        },
        HttpHeader {
            name: "Idempotency-Key".to_owned(),
            value: idempotency_key,
        },
    ];

    ic::print("Antes de armar el argument");

    // let email = Email {
    //     to: "miguetoscano288@gmail.com".to_owned(),
    //     subject: "Test email".to_owned(),
    //     body: "Test email body".to_owned(),
    // };

    // let test = body.clone().into_bytes();
    // ic::print("hace bien el into_bytes()");

    // ic::print(format!("{}", email.to_courier_format()));
    // ic::print(format!("{}", email));

    let request = CanisterHttpRequestArgument {
        url: host,
        method: HttpMethod::POST,
        body: Some(email.to_owned().into_bytes()),
        max_response_bytes: None,
        transform: Some(TransformContext::new(transform, vec![])),
        headers: request_headers,
    };

    ic::print(format!("{:?}", request));

    match http_request(request).await {
        Ok((response,)) => {
            ic::print(format!("{:?}", response));
        }
        Err((r, m)) => {
            ic::print(format!("{:?} {:?}", r, m));
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
