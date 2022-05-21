use chrono::Utc;
use rust_chain::core::block::Block;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

fn main() {
    let mut value = 128;
    let mut hasher = DefaultHasher::new();

    value.hash(&mut hasher);
    let firstBlock = Block::new(Utc::now().timestamp_millis(), 0, hasher.finish(), value);

    println!("First block of rust-blockchain: {:#?}", firstBlock);
}
