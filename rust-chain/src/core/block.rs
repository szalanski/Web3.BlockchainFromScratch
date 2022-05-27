use super::hash::Hash;
use super::hash::HashValue;
use chrono::Utc;
use sha2::Digest;
use sha2::Sha256;

#[derive(Debug)]
pub struct Block<T: Default + Hash> {
    timestamp: String,
    lastHash: HashValue,
    hash: HashValue,
    data: T,
}

impl<T> Block<T>
where
    T: Default + Hash,
{
    pub fn new(timestamp: String, lastHash: HashValue, hash: HashValue, data: T) -> Block<T> {
        Block {
            timestamp,
            hash,
            lastHash,
            data,
        }
    }

    pub fn genesis() -> Block<T> {
        Block {
            timestamp: "2022-05-27 19:52:30.513083300 UTC".into(),
            lastHash: Sha256::digest("genesis").into(),
            hash: Sha256::digest("genesis").into(),
            data: T::default(),
        }
    }

    pub fn mine_block(lastBlock: Block<T>, data: T) -> Block<T> {
        let timestamp = Utc::now().to_string();
        let lastHash = lastBlock.hash;
        let hash = Block::hash(&timestamp, &lastBlock.hash, &data);
        Block::new(timestamp, lastHash, hash, data)
    }

    pub fn hash(timestamp: &String, lastHash: &HashValue, data: &T) -> HashValue {
        let mut hasher = Sha256::new();
        hasher.update(timestamp);
        hasher.update(lastHash);
        hasher.update(data.hash());
        hasher.finalize().into()
    }
}
