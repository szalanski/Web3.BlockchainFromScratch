use super::block::Block;
use super::blockchain::Blockchain;
use super::test_data::Data;

#[test]
fn blockchain_starts_witch_genesis_block() {
    let genesis: Block<Data> = Block::genesis();
    let bc: Blockchain<Data> = Blockchain::new();

    assert!(bc.chain[0].data == genesis.data);
    assert!(bc.chain[0].hash == genesis.hash);
    assert!(bc.chain[0].last_hash == genesis.last_hash);
    assert!(bc.chain[0].timestamp == genesis.timestamp);
}
