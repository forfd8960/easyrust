pub mod errors;
pub mod pb;
pub mod service;
pub mod storage;

pub use errors::KvError;
pub use pb::abi::*;
pub use service::*;
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

    fn test_basic_interface(store: impl Storage) {
        let val = store.set("t1", "good".into(), "day".into());
        assert_eq!(val.unwrap().is_none(), true);

        let val1 = store.set("t1", "good".into(), "day1".into());
        assert_eq!(val1, Ok(Some("day".into())));

        let v = store.get("t1", "good");
        assert_eq!(v, Ok(Some("day1".into())));

        let val2 = store.get("t1", "food");
        assert_eq!(Ok(None), val2);

        assert_eq!(store.contains("t1", "good"), Ok(Some(true)));
        assert_eq!(store.contains("t1", "good1"), Ok(Some(false)));

        let v = store.del("t1", "good");
        assert_eq!(v, Ok(Some("day1".into())));
        assert_eq!(Ok(None), store.del("t1", "good"));
    }
}
