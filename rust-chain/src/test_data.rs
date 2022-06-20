use super::{hash::Hash, hash::HashValue};
use sha2::{Digest, Sha256};

///Data type under test.
#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
pub struct Data {
    pub field: i32,
}

impl Hash for Data {
    fn hash(&self) -> HashValue {
        Sha256::digest(self.field.to_string()).into()
    }
}
