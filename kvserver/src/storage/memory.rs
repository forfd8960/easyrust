use crate::{KvError, Kvpair, Storage, Value};
use dashmap::{mapref::one::Ref, DashMap};

#[derive(Debug, Clone, Default)]
pub struct MemTable {
    tables: DashMap<String, DashMap<String, Value>>,
}

impl MemTable {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_or_create_table(&self, name: &str) -> Ref<String, DashMap<String, Value>> {
        match self.tables.get(name) {
            Some(table) => table,
            None => {
                let entry = self.tables.entry(name.into()).or_default();
                entry.downgrade()
            }
        }
    }
}

impl Storage for MemTable {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.get(key).map(|v| v.value().clone()))
    }

    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table(table);
        let result = table.insert(key, value);
        Ok(result)
    }

    fn contains(&self, table: &str, key: &str) -> Result<Option<bool>, KvError> {
        let table = self.get_or_create_table(table);
        let result = table.get(key);
        Ok(Some(result.is_some()))
    }

    fn del(&self, table_name: &str, key: &str) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table(table_name);
        let result = table.remove(key);
        match result {
            Some(kv) => Ok(Some(kv.1)),
            None => Err(KvError::NotFound(table_name.to_string(), key.to_string())),
        }
    }

    fn get_all(&self, table_name: &str) -> Result<Vec<Kvpair>, KvError> {
        let table = self.get_or_create_table(table_name);
        let iter = table
            .iter()
            .map(|kv| Kvpair::new(kv.key(), kv.value().clone()));
        Ok(Vec::from_iter(iter))
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
