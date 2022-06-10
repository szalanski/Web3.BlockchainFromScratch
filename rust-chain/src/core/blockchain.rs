use super::block::Block;
use super::hash::Hash;

pub struct Blockchain<T: Default + Hash> {
    pub chain: Vec<Block<T>>,
}

impl<T> Blockchain<T>
where
    T: Default + Hash,
{
    pub fn new() -> Blockchain<T> {
        Blockchain {
            chain: vec![Block::genesis()],
        }
    }

    pub fn add(&mut self, data: T) {
        let last_block = self.chain.last().unwrap();
        let mut block = Block::mine_block(last_block, data);
        self.chain.push(block);
    }
}
