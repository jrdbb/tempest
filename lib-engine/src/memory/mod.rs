mod std_btree;

use std::sync::Arc;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Abort,
    Internal(String),
    Value(String),
}

pub trait Store {
    fn set(&mut self, key: &[u8], value: Vec<u8>) -> Result<()>;

    fn get(&self, key: &[u8]) -> Result<Option<Arc<Vec<u8>>>>;

    fn delete(&mut self, key: &[u8]) -> Result<()>;
}