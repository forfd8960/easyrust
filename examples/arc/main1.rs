use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut handlers = vec![];
    let number = new_arc(0);

    for _ in 0..2 {
        let n_clone = make_clone(&number);
        let handler = thread::spawn(move || {
            for _ in 0..10 {
                let mut val = n_clone.lock().unwrap();
                *val += 1;
            }
        });
        handlers.push(handler);
    }

    handlers.into_iter().for_each(|h| h.join().unwrap());
    assert_eq!(*number.lock().unwrap(), 20);
    println!("{:?}", number);
}

fn new_arc(n: i32) -> Arc<Mutex<i32>> {
    Arc::new(Mutex::new(n))
}

fn make_clone(input: &Arc<Mutex<i32>>) -> Arc<Mutex<i32>> {
    Arc::clone(&input)
}
