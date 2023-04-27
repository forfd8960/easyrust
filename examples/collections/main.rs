use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    let m1 = hash_map();
    println!("{}", get_value_from_hash("rust", m1));

    let m = btree_map();
    println!("{}", get_value("rust", m));

    let words = vec!["this", "is", "a", "good", "day", "good", "day"];
    let words_count = count_words(words);
    // words_count: {"day": 2, "this": 1, "a": 1, "good": 2, "is": 1}
    println!("words_count: {:?}", words_count);

    binary_heap();
    hash_set();
    vec_queue();
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

struct Job {
    name: String,
    priority: i32,
}

fn binary_heap() {
    let mut bin_heap = BinaryHeap::<(String, i32)>::new();
    bin_heap.push(("writing rust".to_string(), 1));
    bin_heap.push(("writing golang".to_string(), 1));
    bin_heap.push(("writing python".to_string(), 1));
    bin_heap.push(("writing java".to_string(), 1));

    while let Some(v) = bin_heap.pop() {
        println!("job: {:?}", v);
    }
}

fn hash_set() {
    let mut num_set = HashSet::<i32>::new();
    num_set.insert(1);
    num_set.insert(1);
    num_set.insert(3);
    num_set.insert(2);
    num_set.insert(2);
    num_set.insert(8);

    println!("{}", num_set.get(&1).unwrap());
    println!("{}", num_set.get(&3).unwrap());
    println!("{:?}", num_set.get(&6));
    println!("{:?}", num_set.get(&8).is_none());
}

fn vec_queue() {
    let mut task_queue = VecDeque::<(&str, bool)>::new();
    let tasks = vec![("task1", false), ("task2", false)];
    for t in tasks {
        task_queue.push_front(t);
    }

    let mut t1 = task_queue.pop_front().unwrap();
    t1.1 = true;
    println!("{:?} is done", t1);
    println!("{:?} is not done", task_queue.pop_front().unwrap());
}

fn get_value_from_hash(key: &str, m: HashMap<&str, u32>) -> u32 {
    *m.get(key).unwrap()
}

fn get_value(key: &str, m: BTreeMap<&str, u32>) -> u32 {
    *m.get(key).unwrap()
}
