use acme::data::Identity;

use crate::blockchain::block::Block;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ChainStates {
    Initializing,
    Initialized,
    Terminating,
    Terminated,
    Computing,
    Configuring,
    Validating,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Chain {
    pub id: Identity,
    pub hash: String,
    pub key: String,
    pub state: ChainStates,
    pub store: Vec<Block>,  // Block Store
}

impl Chain {
    pub fn new(hash: String, key: String, state: ChainStates, store: Vec<Block>) -> Self {
        Self {
            id: Identity::new(),
            hash,
            key,
            state,
            store,
        }
    }
}