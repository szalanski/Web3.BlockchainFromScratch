use rust_chain::core::block::Block;
use rust_chain::core::hash::{Hash, HashValue};
use sha2::{Digest, Sha256};

#[derive(Debug, Default)]
struct Data;

impl Hash for Data {
    fn hash(&self) -> HashValue {
        Sha256::digest("dummy").into()
    }
}

fn main() {
    let genesis: Block<Data> = Block::genesis();
    println!("Genesis block for rust-blockchain: {:#?}", genesis);
    let minedBlock = Block::mine_block(genesis, Data);
    println!("First mined block: {:#?}", minedBlock);
}
