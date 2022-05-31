use acme::data::{Clock, Identity};
use serde::{Deserialize, Serialize};

pub type Transaction = std::collections::HashMap<String, Vec<String>>;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Block {
    pub id: Identity,
    pub hash: String,
    pub nonce: String,
    pub previous: String,
    pub timestamp: Clock,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(hash: String, nonce: String, previous: String, transactions: Vec<Transaction>) -> Self {
        Self {
            id: Identity::new(),
            hash,
            nonce,
            previous,
            timestamp: chrono::Local::now().into(),
            transactions,
        }
    }
}