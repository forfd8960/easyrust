use std::cmp::{Ord, Ordering};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
struct Item<'a> {
    desc: &'a str,
}

impl Display for Item<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.desc)
    }
}

#[derive(Debug)]
struct MyGenericStruct<T: Display, U: Ord + Display> {
    name: T,
    data: U,
}

impl<T, U> MyGenericStruct<T, U>
where
    T: Display,
    U: Ord + Display,
{
    fn new(name: T, data: U) -> Self {
        MyGenericStruct {
            name: name,
            data: data,
        }
    }

    fn cmp(&self, other: MyGenericStruct<T, U>) -> i8 {
        match self.data.cmp(&other.data) {
            Ordering::Equal => 0,
            Ordering::Less => -1,
            Ordering::Greater => 1,
        }
    }
}

fn main() {
    display_it(5);
    display_it("five");
    display_it("abc".to_string());
    display_it(10.00);
    display_it(Item {
        desc: "this is a item",
    });

    let obj1 = MyGenericStruct::new("abc", 100);
    println!("{:?}", obj1);

    let data2 = MyGenericStruct::new("abc", 99);
    println!("compare obj2 and obj2: {}", obj1.cmp(data2));
}

fn display_it<T: Display>(num: T) -> T {
    println!("{}", num);
    num
}
