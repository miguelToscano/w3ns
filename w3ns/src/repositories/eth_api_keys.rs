use crate::domain::api_keys::types::{EthApiKey};
use std::collections::HashMap;

#[derive(Default)]
pub struct EthApiKeys(HashMap<String, EthApiKey>);

impl EthApiKeys {
    pub fn archive(&mut self) -> Vec<(String, EthApiKey)> {
        let map = std::mem::replace(&mut self.0, HashMap::new());
        map.into_iter().collect()
    }

    pub fn load(&mut self, archive: Vec<(String, EthApiKey)>) {
        self.0 = archive.into_iter().collect();
    }

    pub fn create(&mut self, api_key: &EthApiKey) -> Result<(), ()> {
        self.0.insert(api_key.clone().owner, api_key.clone());
        Ok(())
    }

    pub fn delete(&mut self, owner: &String) -> Result<(), ()> {
        self.0.remove(owner);
        Ok(())
    }

    pub fn get(&self, owner: &String) -> Option<EthApiKey> {
        self.0.get(owner).cloned()
    }

    pub fn get_all(&self) -> Vec<EthApiKey> {
        self.0.clone().into_values().collect()
    }
}
