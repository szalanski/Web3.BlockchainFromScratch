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

#[test]
fn blockchain_chain_validation_is_valid() {
    let mut other: Blockchain<Data> = Blockchain::new();
    other.add(Data { field: 11 });

    assert!(Blockchain::is_valid_chain(&other));
}

#[test]
fn blockchain_chain_validation_genesis_block_is_not_valid() {
    let mut other: Blockchain<Data> = Blockchain::new();
    other.add(Data { field: 12 });
    //corrupt genesis block
    other.chain[0].data = Data { field: 12 };
    assert!(Blockchain::is_valid_chain(&other) == false);
}

#[test]
fn blockchain_chain_validation_block_is_not_valid() {
    let mut other: Blockchain<Data> = Blockchain::new();
    other.add(Data { field: 12 });
    other.chain[1].data = Data { field: 10 };

    assert!(Blockchain::is_valid_chain(&other) == false);
}
