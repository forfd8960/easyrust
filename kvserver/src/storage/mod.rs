pub mod memory;

use crate::errors::KvError;
use crate::pb::abi::{Kvpair, Value};

pub trait Storage {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    fn set(&self, table: &str, key: &str, value: &str) -> Result<Option<Value>, KvError>;
    fn contains(&self, table: &str, key: &str) -> Result<Option<bool>, KvError>;
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    fn get_all(&self, table: &str) -> Result<Vec<Value>, KvError>;
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
}
