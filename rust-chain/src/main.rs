use chrono::Utc;
use rust_chain::core::block::Block;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Debug, Hash, Default)]
struct Data;

fn main() {
    let genesis: Block<Data> = Block::genesis();
    println!("Genesis block for rust-blockchain: {:#?}", genesis);
    let minedBlock = Block::mine_block(genesis, Data);
    println!("First mined block: {:#?}", minedBlock);
}

fn new_block_example() {
    let mut value = 128;
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);

    let firstBlock = Block::new(Utc::now().timestamp_millis(), 0, hasher.finish(), value);
    println!("First block of rust-blockchain: {:#?}", firstBlock);
}
