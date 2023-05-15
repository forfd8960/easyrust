use crate::{KvError, Kvpair, Storage, Value};
use dashmap::DashMap;

#[derive(Debug, Clone, Default)]
pub struct MemTable {
    tables: DashMap<String, DashMap<String, Value>>,
}

impl MemTable {
    pub fn new() -> Self {
        MemTable {
            tables: DashMap::new(),
        }
    }
}

impl Storage for MemTable {
    fn set(&self, table: &str, key: &str, value: &str) -> Result<Option<Value>, KvError> {
        Ok(None)
    }

    fn contains(&self, table: &str, key: &str) -> Result<Option<bool>, KvError> {
        Ok(None)
    }

    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        Ok(None)
    }

    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        Ok(None)
    }

    fn get_all(&self, table: &str) -> Result<Vec<Value>, KvError> {
        Ok(vec![])
    }

    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError> {
        // mismatched types
        // expected mutable reference `&mut dyn Iterator<Item = abi::Kvpair>`
        // let iterator: &mut dyn Iterator<Item = Kvpair> = self;
        // Ok(Box::new(iterator))
        Err(KvError::NotImplemented(
            "method not implemented yet".to_string(),
        ))
    }
}

impl Iterator for MemTable {
    type Item = Kvpair;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
