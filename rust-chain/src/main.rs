use chrono::Utc;
use rust_chain::core::block::{Block, Hash, Hashable};
use sha2::{Digest, Sha256};

#[derive(Debug, Default)]
struct Data;

impl Hashable for Data {
    fn hash(&self) -> Hash {
        Sha256::digest("dummy").into()
    }
}

fn main() {
    let genesis: Block<Data> = Block::genesis();
    println!("Genesis block for rust-blockchain: {:#?}", genesis);
    let minedBlock = Block::mine_block(genesis, Data);
    println!("First mined block: {:#?}", minedBlock);
}
