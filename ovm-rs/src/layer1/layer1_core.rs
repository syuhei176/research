use super::event::Event;
use super::tx::Transaction;
use crate::layer2::Claim;

pub struct Layer1Core {}

impl Layer1Core {
    pub fn submit(_tx: Transaction) {}

    pub fn get_events() -> Vec<Event> {
        vec![]
    }

    pub fn claim_property(_claim: Claim) {}
}
