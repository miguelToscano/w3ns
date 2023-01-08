use crate::domain::topics::types::Topic;
use ic_kit::candid::Principal;
use std::{collections::HashMap, vec};

#[derive(Default)]
pub struct Topics(HashMap<Principal, Vec<Topic>>);

impl Topics {
    pub fn archive(&mut self) -> Vec<(Principal, Vec<Topic>)> {
        let map = std::mem::replace(&mut self.0, HashMap::new());
        map.into_iter().collect()
    }

    pub fn load(&mut self, archive: Vec<(Principal, Vec<Topic>)>) {
        self.0 = archive.into_iter().collect();
    }

    pub fn add(&mut self, owner: &Principal, topic: &Topic) -> Result<(), ()> {
        let mut owner_topics = self.0.get(owner).unwrap_or(&vec![]).clone();

        if owner_topics
            .iter()
            .find(|owner_topic| owner_topic.name == topic.name)
            .is_none()
        {
            owner_topics.push(topic.clone());
        }

        self.0.insert(*owner, owner_topics);

        Ok(())
    }

    // Provided topic has to exist
    pub fn add_topic_subscriber(
        &mut self,
        owner: &Principal,
        topic_name: String,
        subscriber: String,
    ) -> Result<(), ()> {
        let topic = self.get_topic(owner, topic_name.clone()).unwrap();

        let mut updated_subscribers = topic.subscribers.clone();
        updated_subscribers.push(subscriber);
        updated_subscribers.dedup();

        let updated_topic = Topic {
            name: topic.name.clone(),
            subscribers: updated_subscribers,
            created_at: topic.created_at,
            owner: topic.owner,
        };

        self.delete(owner, topic_name)?;
        self.add(owner, &updated_topic)?;

        Ok(())
    }

    // Assumes the topic for the given owner exists
    pub fn delete(&mut self, owner: &Principal, topic_name: String) -> Result<(), ()> {
        let owner_topics = self.0.get(owner).unwrap();

        let updated_topics = owner_topics
            .clone()
            .iter()
            .filter(|&topic| topic.name != topic_name)
            .map(|topic| topic.clone())
            .collect::<Vec<Topic>>();

        // If the removed topic was the only one, delete the whole entry
        if updated_topics.len() == 0 {
            self.0.remove(owner);
        } else {
            self.0.insert(*owner, updated_topics);
        }

        Ok(())
    }

    pub fn get_topics(&self, owner: &Principal) -> Vec<Topic> {
        self.0.get(owner).unwrap_or(&vec![]).to_vec()
    }

    pub fn get_topic(&self, owner: &Principal, topic_name: String) -> Option<&Topic> {
        let owner_topics = self.0.get(owner);

        if owner_topics.is_none() {
            return None;
        }

        owner_topics
            .unwrap()
            .iter()
            .find(|&topic| topic.name == topic_name)
    }

    pub fn get_all(&self) -> Vec<Vec<Topic>> {
        self.0.clone().into_values().collect()
    }
}
