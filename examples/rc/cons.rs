use std::rc::Rc;
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

/*
strong count of a: 1
link to b, strong count of a: 2
link to c, strong count of a: 3
*/

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    println!("strong count of a: {}", Rc::strong_count(&a)); // 1

    // create new pointer to a, increase strong refer count
    let b = Cons(3, Rc::clone(&a));
    println!("link to b, strong count of a: {}", Rc::strong_count(&a)); // 2
                                                                        // create new pointer to a
    let c = Cons(4, Rc::clone(&a));
    println!("link to c, strong count of a: {}", Rc::strong_count(&a)); // 3

    // a: Cons(5, Cons(10, Nil)), b: Cons(3, Cons(5, Cons(10, Nil))), c: Cons(4, Cons(5, Cons(10, Nil)))
    println!("a: {:?}, b: {:?}, c: {:?}", a, b, c);
}
