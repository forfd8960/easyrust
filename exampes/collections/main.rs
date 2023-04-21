use std::collections::{BTreeMap, HashMap};

fn main() {
    let m1 = hash_map();
    println!("{}", get_value_from_hash("rust", m1));

    let m = btree_map();
    println!("{}", get_value("rust", m));

    let words = vec!["this", "is", "a", "good", "day", "good", "day"];
    let words_count = count_words(words);
    // words_count: {"day": 2, "this": 1, "a": 1, "good": 2, "is": 1}
    println!("words_count: {:?}", words_count);
}

fn count_words(words: Vec<&str>) -> HashMap<&str, u32> {
    let mut words_count = HashMap::<&str, u32>::new();
    for word in words {
        let count = words_count.entry(word).or_insert(0);
        *count += 1;
    }
    words_count
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
