use ic_kit::candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Push {
    pub firebase_token: String,
    pub title: String,
    pub body: String,
}

impl Push {
    pub fn to_courier_format(&self) -> String {
        format!("{{ \"message\": {{ \"to\": {{\"firebaseToken\":\"{}\"}}, \"content\": {{ \"title\": \"{}\", \"body\": \"{}\" }} }} }}", self.firebase_token, self.title, self.body)
    }
}
