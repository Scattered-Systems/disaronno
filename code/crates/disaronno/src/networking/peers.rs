use libp2p;

pub mod types {
    use libp2p;

    pub type AuthNoiseKey = libp2p::noise::AuthenticKeypair<CryptoSpec>;
    pub type CryptoSpec = libp2p::noise::X25519Spec;
    pub type NoiseKey = libp2p::noise::Keypair<CryptoSpec>;
    pub type PeerId = libp2p::PeerId;
    pub type PeerKey = libp2p::identity::Keypair;
}


#[derive(Clone, Debug)]
pub struct Peer {
    pub id: types::PeerId,
    pub key: types::PeerKey
}

impl Peer {
    pub fn new() -> Self {
        let key = types::PeerKey::generate_ed25519();
        Self {
            id: types::PeerId::from(key.public().clone()),
            key: key.clone()
        }
    }
}

impl std::fmt::Display for Peer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Peer(id={})", self.id)
    }
}