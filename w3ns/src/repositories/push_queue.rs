use crate::domain::push::types::{QueuedPush};

#[derive(Default)]
pub struct PushQueue(Vec<QueuedPush>);

impl PushQueue {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn archive(&mut self) -> Vec<QueuedPush> {
        let vec = std::mem::replace(&mut self.0, Vec::new());
        vec
    }

    pub fn load(&mut self, archive: Vec<QueuedPush>) {
        self.0 = archive;
    }

    pub fn enqueue(&mut self, email: QueuedPush) {
        self.0.push(email);
    }

    pub fn dequeue(&mut self) -> Option<QueuedPush> {
        self.0.pop()
    }

    pub fn dequeue_all(&mut self) -> Vec<QueuedPush> {
        let mut queued_push = vec![];

        for _ in 0..self.0.len() {
            queued_push.push(self.dequeue().unwrap());
        }

        queued_push
    }

    pub fn get(&self, index: usize) -> Option<&QueuedPush> {
        self.0.get(index)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get_all(&self) -> Vec<QueuedPush> {
        self.0.clone()
    }
}
