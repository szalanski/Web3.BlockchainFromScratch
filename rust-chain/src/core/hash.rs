pub type HashValue = [u8; 32];

pub trait Hash {
    fn hash(&self) -> HashValue;
}
