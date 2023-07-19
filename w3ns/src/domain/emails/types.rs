use ic_kit::candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct SendEmailInput {
    pub to: String,
    pub title: String,
    pub body: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct SendEthEmailInput {
    pub address: String,
    pub to: String,
    pub title: String,
    pub body: String,
}

impl From<SendEthEmailInput> for SendEmailInput {
    fn from(input: SendEthEmailInput) -> Self {
        Self {
            to: input.to,
            title: input.title,
            body: input.body,
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct QueuedEmail {
    pub api_key: String,
    pub to: String,
    pub title: String,
    pub body: String,
}

impl SendEmailInput {
    pub fn to_courier_format(&self) -> String {
        format!("{{ \"message\": {{ \"to\": {{\"email\":\"{}\"}}, \"content\": {{ \"title\": \"{}\", \"body\": \"{}\" }} }} }}", self.to, self.title, self.body)
    }
}
