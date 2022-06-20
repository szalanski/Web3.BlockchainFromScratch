use super::block::Block;
use super::hash::Hash;
use log::{info, warn};

#[derive(Debug, Clone)]
pub struct Blockchain<T: Default + Hash + Eq> {
    pub chain: Vec<Block<T>>,
}

impl<T> Blockchain<T>
where
    T: Default + Hash + Eq,
{
    pub fn new() -> Blockchain<T> {
        Blockchain {
            chain: vec![Block::genesis()],
        }
    }

    pub fn add(&mut self, data: T) {
        let last_block = self.chain.last().unwrap();
        let block = Block::mine_block(last_block, data);
        self.chain.push(block);
    }

    pub fn is_valid_chain(bc: &Blockchain<T>) -> bool {
        if bc.chain[0] != Block::genesis() {
            return false;
        }

        for i in 1..bc.chain.len() {
            let block = &bc.chain[i];
            let last = &bc.chain[i - 1];

            if block.last_hash != last.hash || block.hash != Block::block_hash(&block) {
                return false;
            }
        }

        true
    }

    pub fn replace_chain(&mut self, other: Blockchain<T>) {
        if other.chain.len() <= self.chain.len() {
            info!("Received chain is not longer than the current chain");
            return;
        }

        if !Blockchain::is_valid_chain(&other) {
            warn!("The received chain is not valid.");
            return;
        }

        info!("Replacing current chain with the new chain.");
        self.chain = other.chain;
    }
}
