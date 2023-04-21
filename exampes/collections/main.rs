use std::collections::{BTreeMap, HashMap};

fn main() {
    let m1 = hash_map();
    println!("{}", get_value_from_hash("rust", m1));

    let m = btree_map();
    println!("{}", get_value("rust", m));
}

fn hash_map() -> HashMap<&'static str, u32> {
    let mut words_count = HashMap::<&str, u32>::new();
    words_count.insert("hello", 1);
    words_count.insert("world", 2);
    words_count.insert("rust", 3);
    words_count.insert("good", 5);
    words_count.insert("day", 6);

    println!("{:?}", words_count);
    words_count
}

fn btree_map() -> BTreeMap<&'static str, u32> {
    let mut words_count = BTreeMap::<&str, u32>::new();
    words_count.insert("hello", 1);
    words_count.insert("world", 2);
    words_count.insert("rust", 3);
    words_count.insert("good", 5);
    words_count.insert("day", 6);

    println!("{:?}", words_count);
    return words_count;
}

fn get_value_from_hash(key: &str, m: HashMap<&str, u32>) -> u32 {
    *m.get(key).unwrap()
}

fn get_value(key: &str, m: BTreeMap<&str, u32>) -> u32 {
    *m.get(key).unwrap()
}
