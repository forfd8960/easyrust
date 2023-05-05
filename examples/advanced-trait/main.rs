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

fn main() {
    let counter = Counter::new(6);
    for v in counter {
        println!("v: {}", v)
    }
}
