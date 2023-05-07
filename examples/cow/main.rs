/*
Cow - Clone on Write

https://blog.logrocket.com/using-cow-rust-efficient-memory-utilization/

enum Cow<'a, B>where B: 'a + ToOwned + ?Sized,{
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}

Cow is a convenient wrapper that allows us to express the idea of optional ownership in Rust.

Cow is an enum consisting of two parameters — a lifetime parameter 'a , and a type parameter B.

B can represent any type that implements the ToOwned trait, and which may or may not be sized, as we have indicated with the ?Sized trait. Furthermore, if B contains any references, they must live for at least as long as the 'a lifetime.

The variant Borrowed contains an immutable reference to B having a lifetime of 'a. The Owned variant contains the type as specified by the implementation of the ToOwned trait of B.

ToOwned is a trait that can be thought of as a generalized Clone trait. This trait allows us to have an owned instance of a borrowed value by either direct cloning or some other internal implementation of duplication.

 For example, when we are converting &[u8] to a String, we need to allocate memory space only when there is an invalid UTF-8 sequence in the input. In other cases, the input is a valid string and can thus be directly used without needing any duplication. Thus, Cow is used to handle this case.
*/

use std::{borrow::Cow, collections::HashSet};

#[derive(Debug, Clone)]
struct Element {
    id: u32,
}

fn deduplicate_elements<'a>(input: &'a [Element]) -> Cow<'a, [Element]> {
    let mut set = HashSet::<u32>::new();
    let mut has_duplicate = false;
    for item in input {
        if set.contains(&item.id) {
            has_duplicate = true;
        }

        set.insert(item.id);
    }
    if !has_duplicate {
        return Cow::Borrowed(input);
    }

    let mut result = Vec::<Element>::new();
    for item in input {
        if set.contains(&item.id) {
            result.push(item.to_owned());
            set.remove(&item.id);
        }
    }

    Cow::Owned(result)
}

fn main() {
    let elements = vec![
        Element { id: 1 },
        Element { id: 1 },
        Element { id: 2 },
        Element { id: 3 },
        Element { id: 3 },
    ];

    // Owned, cloned: [Element { id: 1 }, Element { id: 2 }, Element { id: 3 }]
    match deduplicate_elements(elements.as_slice()) {
        Cow::Borrowed(v) => {
            println!("Borrowed, no clone: {:?}", v);
        }
        Cow::Owned(v) => {
            println!("Owned, cloned: {:?}", v);
        }
    }

    let elements1 = vec![Element { id: 8 }, Element { id: 9 }, Element { id: 6 }];

    // Borrowed, no clone: [Element { id: 8 }, Element { id: 9 }, Element { id: 6 }]
    match deduplicate_elements(elements1.as_slice()) {
        Cow::Borrowed(v) => {
            println!("Borrowed, no clone: {:?}", v);
        }
        Cow::Owned(v) => {
            println!("Owned, cloned: {:?}", v);
        }
    }
}
