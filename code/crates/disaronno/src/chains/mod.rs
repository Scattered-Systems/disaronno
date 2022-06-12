pub mod blocks;
pub mod blockchain;

pub mod constants {
    pub const DIFFICULTY_PREFIX: &str = "00";
}

pub mod types {
    use crate::types::ObjectId;

    pub type BlockData = String;
    pub type BlockId = ObjectId;
    pub type BlockHash = String;
    pub type BlockNonce = u64;
}

pub mod utils {
    use log::info;
    use serde_json::json;
    use sha2::{Digest, Sha256};

    use crate::types::{LocalTime, TimeStamp};

    use super::constants::DIFFICULTY_PREFIX;
    use super::types::{BlockData, BlockHash, BlockId, BlockNonce};

    // Cacluate the hash of a Block using standard Block parameters
    pub fn calculate_hash(id: BlockId, data: BlockData, nonce: BlockNonce, previous: BlockHash, timestamp: TimeStamp) -> Vec<u8> {
        let data = json!({
            "id": id,
            "data": data,
            "nonce": nonce,
            "previous": previous,
            "timestamp": timestamp
        });
        let mut hasher = Sha256::new();
        hasher.update(data.to_string().as_bytes());
        hasher.finalize().as_slice().to_owned()
    }

    // Represent a Block hash in the proper format for the binary interface
    pub fn hash_to_binary_representation(hash: &[u8]) -> String {
        let mut res: String = String::default();
        for c in hash {
            res.push_str(&format!("{:b}", c));
        }
        res
    }

    // Defines the standard method in which blocks are to be mined
    pub fn mine_block(id: BlockId, data: BlockData, nonce: BlockNonce, previous: BlockHash, timestamp: TimeStamp) -> (u64, String) {
        info!("mining block...");
        let mut nonce = 0;

        loop {
            if nonce % 100000 == 0 {
                info!("nonce: {}", nonce);
            }
            let hash = calculate_hash(id, data.clone(), nonce, previous.clone(), timestamp);
            let binary_hash = hash_to_binary_representation(&hash);
            if binary_hash.starts_with(DIFFICULTY_PREFIX) {
                info!(
                    "mined! nonce: {}, hash: {}, binary hash: {}",
                    nonce,
                    hex::encode(&hash),
                    binary_hash
                );
                return (nonce, hex::encode(hash));
            }
            nonce += 1;
        }
    }
}