use ic_kit::candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct SendSmsInput {
    pub to: String,
    pub message: String,
}

impl SendSmsInput {
    pub fn to_courier_format(&self) -> String {
        format!("{{ \"message\": {{ \"to\": {{\"phone_number\":\"{}\"}}, \"content\": {{\"body\": \"{}\" }} }} }}", self.to, self.message)
    }
}
