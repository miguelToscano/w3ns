use ic_kit::candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Email {
    pub to: String,
    pub subject: String,
    pub body: String,
}

impl Email {
    pub fn to_courier_format(&self) -> String {
        format!("{{ \"message\": {{ \"to\": {{\"email\":\"{}\"}}, \"content\": {{ \"title\": \"{}\", \"body\": \"{}\" }} }} }}", self.to, self.subject, self.body)
    }
}
