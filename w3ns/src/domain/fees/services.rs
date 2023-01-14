use ic_kit::ic;

use crate::domain::fees::types::Fee;
use crate::repositories::fees::Fees;

pub fn get_all() -> Vec<Fee> {
    ic::with(|fees_repository: &Fees| fees_repository.get_all())
}

pub fn get(operation: &String) -> Option<Fee> {
    ic::with(|fees_repository: &Fees| fees_repository.get(operation))
}
