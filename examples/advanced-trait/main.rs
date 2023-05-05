use std::ops::Add;

struct Counter {
    max: u32,
    current: u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        Counter {
            max: max,
            current: 0,
        }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            let cur = self.current;
            self.current += 1;
            return Some(cur);
        }

        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct Millimeters(u32);
#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + rhs.0 * 1000)
    }
}

fn main() {
    let counter = Counter::new(6);
    for v in counter {
        println!("v: {}", v)
    }

    let p1 = Point { x: 1, y: -1 };
    let p2 = Point { x: 2, y: 0 };
    // p1 + p2: Point { x: 3, y: -1 }
    println!("p1 + p2: {:?}", p1 + p2);

    let millmeters = Millimeters(100);
    let meters = Meters(50);
    // Millimeters(50100)
    println!("{:?}", millmeters + meters);
}
