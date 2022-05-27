use chrono::{Date, DateTime, TimeZone, Utc};
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

fn datetime_parse() {
    let timestamp = Utc::now().to_string();
    let dt = timestamp.parse::<DateTime<Utc>>();
    println!("{}", timestamp);
    match dt {
        Ok(time) => println!("{}", time),
        Err(err) => println!("Parse, error{}", err),
    }
}
