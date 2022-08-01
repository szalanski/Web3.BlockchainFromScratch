use rust_chain::hash::{Hash, HashValue};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Default, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Data {
    pub field: i32,
}

impl Hash for Data {
    fn hash(&self) -> HashValue {
        Sha256::digest(self.field.to_string()).into()
    }
}
