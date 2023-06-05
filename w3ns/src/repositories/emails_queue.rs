use crate::domain::emails::types::{QueuedEmail};

#[derive(Default)]
pub struct EmailsQueue(Vec<QueuedEmail>);

impl EmailsQueue {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn archive(&mut self) -> Vec<QueuedEmail> {
        let vec = std::mem::replace(&mut self.0, Vec::new());
        vec
    }

    pub fn load(&mut self, archive: Vec<QueuedEmail>) {
        self.0 = archive;
    }

    pub fn enqueue(&mut self, email: QueuedEmail) {
        self.0.push(email);
    }

    pub fn dequeue(&mut self) -> Option<QueuedEmail> {
        self.0.pop()
    }

    pub fn dequeue_all(&mut self) -> Vec<QueuedEmail> {
        let mut queued_emails = vec![];

        for _ in 0..self.0.len() {
            queued_emails.push(self.dequeue().unwrap());
        }

        queued_emails
    }

    pub fn get(&self, index: usize) -> Option<&QueuedEmail> {
        self.0.get(index)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get_all(&self) -> Vec<QueuedEmail> {
        self.0.clone()
    }
}
