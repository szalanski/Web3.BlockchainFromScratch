use super::test_data::Data;
use super::{hash::Hash, hash::HashValue};
use crate::core::block::Block;
use sha2::{Digest, Sha256};
use std::clone;

#[test]
fn mine_block_generates_second_block_correctly() {
    let data = Data { field: 10 };
    let lastBlock: Block<Data> = Block::genesis();

    let block = Block::mine_block(&lastBlock, data.clone());

    assert!(data == block.data);
    assert!(block.last_hash == lastBlock.hash);
}
