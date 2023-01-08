use ic_kit::{candid::CandidType, ic};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Push {
    pub firebase_token: String,
    pub title: String,
    pub body: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct MultiplePush {
    pub firebase_tokens: Vec<String>,
    pub title: String,
    pub body: String,
}

pub trait ToCourierFormat {
    fn to_courier_format(&self) -> String;
}

impl ToCourierFormat for Push {
    fn to_courier_format(&self) -> String {
        format!("{{ \"message\": {{ \"to\": {{\"firebaseToken\":\"{}\"}}, \"content\": {{ \"title\": \"{}\", \"body\": \"{}\" }} }} }}", self.firebase_token, self.title, self.body)
    }
}

impl ToCourierFormat for MultiplePush {
    fn to_courier_format(&self) -> String {
        let mut firebase_tokens_string = vec![];

        for firebase_token in self.firebase_tokens.clone() {
            firebase_tokens_string.push(format!("{{\"firebaseToken\":\"{}\"}}", firebase_token));
        }

        ic::print(format!("{{ \"message\": {{ \"to\": [{}], \"content\": {{ \"title\": \"{}\", \"body\": \"{}\" }} }} }}", firebase_tokens_string.join(", "), self.title, self.body));

        format!("{{ \"message\": {{ \"to\": [{}], \"content\": {{ \"title\": \"{}\", \"body\": \"{}\" }} }} }}", firebase_tokens_string.join(", "), self.title, self.body)
    }
}
