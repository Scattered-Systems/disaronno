use acme::data::Identity;

use crate::blockchain::chain::Chain;

#[derive(Clone, Debug)]
pub struct Node {
    pub id: Identity,
    pub key: Keychain,
    pub chain: Chain,
    // pub transport: Box<dyn libp2p::Transport>
}

impl Node {
    pub fn new(key: Keychain, chain: Chain) -> Self {
        Self {
            id: Identity::new(),
            key,
            chain,
        }
    }
}