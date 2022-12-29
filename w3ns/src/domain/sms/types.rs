use ic_kit::candid::{CandidType};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Sms {
    pub to: String,
    pub message: String,
}

impl Sms {
    pub fn to_courier_format(&self) -> String {
        format!("{{ \"message\": {{ \"to\": {{\"phone_number\":\"{}\"}}, \"content\": {{\"body\": \"{}\" }} }} }}", self.to, self.message)
    }
}