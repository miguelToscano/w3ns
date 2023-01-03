use ic_kit::candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct PushNotification {
    pub firebase_token: String,
    pub title: String,
    pub body: String,
}

impl PushNotification {
    pub fn to_courier_format(&self) -> String {
        format!("{{ \"message\": {{ \"to\": {{\"firebaseToken\":\"{}\"}},  \"providers\": {{ \"firebase-fcm\": {{ \"override\": {{ \"body\": {{ \"notification\": {{ \"title\": \"{}\", \"body\": \"{}\" }} }} }} }} }} }}", self.firebase_token, self.title, self.body)
    }
}
