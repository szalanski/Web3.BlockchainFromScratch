use rust_chain::core::hash::{Hash, HashValue};
use sha2::{Digest, Sha256};

#[derive(Debug, Default)]
struct Data;

impl Hash for Data {
    fn hash(&self) -> HashValue {
        Sha256::digest("dummy").into()
    }
}
