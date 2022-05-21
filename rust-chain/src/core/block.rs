#[derive(Debug)]
pub struct Block<T> {
    timestamp: i64,
    lastHash: u64,
    hash: u64,
    data: T,
}

impl<T> Block<T> {
    pub const fn new(timestamp: i64, lastHash: u64, hash: u64, data: T) -> Block<T> {
        Block {
            timestamp,
            hash,
            lastHash,
            data,
        }
    }
}
