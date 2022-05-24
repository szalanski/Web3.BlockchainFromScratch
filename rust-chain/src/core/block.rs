#[derive(Debug)]
pub struct Block<T: Default> {
    timestamp: i64,
    lastHash: u64,
    hash: u64,
    data: T,
}

impl<T> Block<T>
where
    T: Default,
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
}
