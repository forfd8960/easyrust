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

fn main() {
    display_it(5);
    display_it("five");
    display_it("abc".to_string());
    display_it(10.00);
    display_it(Item{desc: "this is a item"});
}

fn display_it<T: Display>(num: T) -> T {
    println!("{}", num);
    num
}