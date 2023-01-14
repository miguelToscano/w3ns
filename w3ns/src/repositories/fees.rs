use crate::domain::fees::types::Fee;
use std::collections::HashMap;

#[derive(Default)]
pub struct Fees(HashMap<String, Fee>);

impl Fees {
    pub fn archive(&mut self) -> Vec<(String, Fee)> {
        let map = std::mem::replace(&mut self.0, HashMap::new());
        map.into_iter().collect()
    }

    pub fn load(&mut self, archive: Vec<(String, Fee)>) {
        self.0 = archive.into_iter().collect();
    }

    pub fn create(&mut self, fee: &Fee) -> Result<(), ()> {
        self.0.insert(fee.clone().operation, fee.clone());
        Ok(())
    }

    pub fn delete(&mut self, operation: &String) -> Result<(), ()> {
        self.0.remove(operation);
        Ok(())
    }

    pub fn get(&self, operation: &String) -> Option<Fee> {
        self.0.get(operation).cloned()
    }

    pub fn get_all(&self) -> Vec<Fee> {
        self.0.clone().into_values().collect()
    }
}
