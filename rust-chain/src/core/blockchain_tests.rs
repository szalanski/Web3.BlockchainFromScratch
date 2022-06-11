use super::block::Block;
use super::blockchain::Blockchain;
use super::test_data::Data;

#[test]
fn blockchain_starts_witch_genesis_block() {
    let genesis: Block<Data> = Block::genesis();
    let bc: Blockchain<Data> = Blockchain::new();

    assert!(genesis == bc.chain[0])
}

#[test]
fn blockchain_adds_new_block() {
    let mut bc: Blockchain<Data> = Blockchain::new();
    let test_data = Data { field: 10 };

    bc.add(test_data.clone());

    match bc.chain.last() {
        Some(block) => assert!(
            block.data == test_data,
            "Data not match, expected: {:?}, actual {:?}",
            block.data,
            test_data
        ),
        None => assert!(false, "blockchain returned no block"),
    }
}
