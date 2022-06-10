use super::hash::Hash;
use super::hash::HashValue;
use chrono::Utc;
use sha2::Digest;
use sha2::Sha256;

#[derive(Debug, Clone)]
pub struct Block<T: Default + Hash + Clone> {
    pub timestamp: String,
    pub last_hash: HashValue,
    pub hash: HashValue,
    pub data: T,
}

impl<T> Block<T>
where
    T: Default + Hash + Clone,
{
    pub fn new(timestamp: String, last_hash: HashValue, hash: HashValue, data: T) -> Block<T> {
        Block {
            timestamp,
            hash,
            last_hash,
            data,
        }
    }

    pub fn genesis() -> Block<T> {
        Block {
            timestamp: "2022-05-27 19:52:30.513083300 UTC".into(),
            last_hash: Sha256::digest("genesis").into(),
            hash: Sha256::digest("genesis").into(),
            data: T::default(),
        }
    }

    pub fn mine_block(last_block: &Block<T>, data: T) -> Block<T> {
        let timestamp = Utc::now().to_string();
        let last_hash = last_block.hash;
        let hash = Block::hash(&timestamp, &last_block.hash, &data);
        Block::new(timestamp, last_hash, hash, data)
    }

    pub fn hash(timestamp: &String, last_hash: &HashValue, data: &T) -> HashValue {
        let mut hasher = Sha256::new();
        hasher.update(timestamp);
        hasher.update(last_hash);
        hasher.update(data.hash());
        hasher.finalize().into()
    }
}
