use chrono::Utc;
use sha2::Digest;
use sha2::Sha256;

pub trait Hashable {
    fn hash(&self) -> [u8; 32];
}

pub type Hash = [u8; 32];

#[derive(Debug)]
pub struct Block<T: Default + Hashable> {
    timestamp: i64,
    lastHash: Hash,
    hash: Hash,
    data: T,
}

impl<T> Block<T>
where
    T: Default + Hashable,
{
    pub fn new(timestamp: i64, lastHash: Hash, hash: Hash, data: T) -> Block<T> {
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
            lastHash: Sha256::digest("genesis").into(),
            hash: Sha256::digest("genesis").into(),
            data: T::default(),
        }
    }

    pub fn mine_block(lastBlock: Block<T>, data: T) -> Block<T> {
        let timestamp = Utc::now().timestamp_millis();
        let lastHash = lastBlock.hash;
        let hash = data.hash();
        Block::new(timestamp, lastHash, hash, data)
    }
}
