use std::collections::BTreeMap;
use super::{Store, Result};
use std::sync::Arc;

pub struct BTreeStore {
    data: BTreeMap<Vec<u8>, Arc<Vec<u8>>>,
}

impl Store for BTreeStore {
    fn set(&mut self, key: &[u8], value: Vec<u8>) -> Result<()> {
        self.data.insert(key.to_vec(), Arc::new(value));
        Ok(())
    }

    fn get(&self, key: &[u8]) -> Result<Option<Arc<Vec<u8>>>> {
        Ok(self.data.get(key).cloned())
    }

    fn delete(&mut self, key: &[u8]) -> Result<()> {
        self.data.remove(key);
        Ok(())
    }
}