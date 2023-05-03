use std::{
    sync::{Arc, Mutex},
    thread,
};

/*
Thread 2 is working - increase data2
Thread 1 is working - increase data1
Thread 2 is working - increase data2
Thread 2 is working - increase data2
Thread 2 is working - increase data2
Thread 2 is working - increase data2
Thread 2 is working - increase data2
Thread 2 is working - increase data2
Thread 2 is working - increase data2
Thread 2 is working - increase data2
Thread 2 is working - increase data2
Thread 1 is working - increase data1
Thread 1 is working - increase data1
Thread 1 is working - increase data1
Thread 1 is working - increase data1
Thread 1 is working - increase data1
Thread 1 is working - increase data1
Thread 1 is working - increase data1
Thread 1 is working - increase data1
Thread 1 is working - increase data1
data: Mutex { data: 20, poisoned: false, .. }
*/

fn main() {
    let data = Arc::new(Mutex::new(0));
    let data1 = Arc::clone(&data);
    let data2 = Arc::clone(&data);

    let handle = thread::spawn(move || {
        for _ in 0..10 {
            println!("Thread 1 is working - increase data1");
            *data1.lock().unwrap() += 1;
        }
    });
    let handle1 = thread::spawn(move || {
        for _ in 0..10 {
            println!("Thread 2 is working - increase data2");
            *data2.lock().unwrap() += 1;
        }
    });
    handle1.join().unwrap();
    handle.join().unwrap();
    println!("data: {:?}", data);
}
