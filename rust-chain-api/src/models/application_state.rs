use super::data::Data;
use rust_chain::blockchain::Blockchain;

pub type BlockchainType = Blockchain<Data>;

pub struct ApplicationState {
    pub bc: BlockchainType,
}

impl ApplicationState {
    pub fn new() -> ApplicationState {
        ApplicationState {
            bc: BlockchainType::new(),
        }
    }
}
