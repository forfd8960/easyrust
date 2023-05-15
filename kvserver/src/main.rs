pub mod errors;
pub mod pb;
pub mod service;
pub mod storage;

pub use errors::KvError;
pub use pb::abi::*;
pub use storage::*;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::storage::memory::*;

    #[test]
    fn memtable_basic_interface() {
        let store = MemTable::new();
        test_basic_interface(store);
    }

    fn test_basic_interface(store: impl Storage) {}
}
