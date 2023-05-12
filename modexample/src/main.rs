use crate::garden::vegetables::Broccoli;

pub mod garden;

fn main() {
    let broc = Broccoli {};
    println!("{:?}", broc);
}
