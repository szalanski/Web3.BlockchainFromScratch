use super::test_data::Data;
use crate::core::block::Block;

#[test]
fn mine_block_generates_second_block_correctly() {
    let data = Data { field: 10 };
    let last_block: Block<Data> = Block::genesis();

    let block = Block::mine_block(&last_block, data.clone());

    assert!(data == block.data);
    assert!(block.last_hash == last_block.hash);
}
