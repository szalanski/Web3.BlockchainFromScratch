use chrono::Utc;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Debug)]
pub struct Block<T: Default + Hash> {
    timestamp: i64,
    lastHash: u64,
    hash: u64,
    data: T,
}

impl<T> Block<T>
where
    T: Default + Hash,
{
    pub fn new(timestamp: i64, lastHash: u64, hash: u64, data: T) -> Block<T> {
        Block {
            timestamp,
            hash,
            lastHash,
            data,
        }
    }

    pub fn genesis() -> Block<T> {
        Block {
            timestamp: 0,
            lastHash: 0,
            hash: 12076202582256590650,
            data: T::default(),
        }
    }

    pub fn mine_block(lastBlock: Block<T>, data: T) -> Block<T> {
        let timestamp = Utc::now().timestamp_millis();
        let lastHash = lastBlock.hash;
        let hash = Block::generate_hash(&data);
        Block::new(timestamp, lastHash, hash, data)
    }

    fn generate_hash(data: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        hasher.finish()
    }
}
