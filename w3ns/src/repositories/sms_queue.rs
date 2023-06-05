use crate::domain::sms::types::{QueuedSms, SendSmsInput};

#[derive(Default)]
pub struct SmsQueue(Vec<QueuedSms>);

impl SmsQueue {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn archive(&mut self) -> Vec<QueuedSms> {
        let vec = std::mem::replace(&mut self.0, Vec::new());
        vec
    }

    pub fn load(&mut self, archive: Vec<QueuedSms>) {
        self.0 = archive;
    }

    pub fn enqueue(&mut self, sms: QueuedSms) {
        self.0.push(sms);
    }

    pub fn dequeue(&mut self) -> Option<QueuedSms> {
        self.0.pop()
    }

    pub fn dequeue_all(&mut self) -> Vec<QueuedSms> {
        let mut queued_sms = vec![];

        for _ in 0..self.0.len() {
            queued_sms.push(self.dequeue().unwrap());
        }

        queued_sms
    }

    pub fn get(&self, index: usize) -> Option<&QueuedSms> {
        self.0.get(index)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get_all(&self) -> Vec<QueuedSms> {
        self.0.clone()
    }
}