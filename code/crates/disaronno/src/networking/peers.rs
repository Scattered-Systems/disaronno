use libp2p::{
    noise::{AuthenticKeypair, Keypair, X25519Spec},
    PeerId,
};

#[derive(Clone)]
pub struct Peer {
    pub id: PeerId,
    pub key: Keypair,
    pub noise: AuthenticKeypair<X25519Spec>,
}

impl Peer {
    pub fn new() -> Result<Self, libp2p::noise::NoiseError> {
        let key = Keychain::generate_ed25519();
        let noise = Keypair::<X25519Spec>::new()
            .into_authentic(&key)
            .expect("Signing libp2p-noise static DH keypair failed.");
        Ok(
            Self {
                id: PeerId::from(key.public()),
                key: key.clone(),
                noise: noise.clone(),
            }
        )
    }

    pub fn from(key: Keychain) -> Self {
        let noise = Keypair::<X25519Spec>::new()
            .into_authentic(&key)
            .expect("Signing libp2p-noise static DH keypair failed.");
        Self {
            id: PeerId::from(key.public()),
            key: key.clone(),
            noise: noise.clone(),
        }
    }
}

impl std::fmt::Display for Peer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Peer(id={})", self.id)
    }
}