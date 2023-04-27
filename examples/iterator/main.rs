#[derive(Debug)]
enum Programmer {
    Rust,
    Go,
    Js,
    Python,
}

struct ItCompany {
    name: String,
    programmers: Vec<Programmer>,
}

impl Iterator for ItCompany {
    type Item = Programmer;
    fn next(&mut self) -> Option<Self::Item> {
        self.programmers.pop()
    }
}

fn main() {
    let origin = vec![1, 6, 8];

    let vec1 = origin.iter().map(|x| x + 1).collect::<Vec<i32>>();
    println!("{:?}", vec1);

    let vec2 = origin.into_iter().map(|x| x * 10).collect::<Vec<i32>>();
    println!("{:?}", vec2);

    let mut mut_vec = vec![10, 80, 55];
    mut_vec.iter_mut().for_each(|x| *x += 10);
    println!("{:?}", mut_vec); // [20, 90, 65]

    run_iter();
    run_into_iter();
    iter_next();
    iter_over_company();
}

fn run_iter() {
    let my_vec = vec!["a", "b", "c"];
    let vec1 = my_vec.iter().map(|x| format!("{}-*", x));

    for s in vec1 {
        println!("{}", s);
    }

    let my_vec1 = vec![3, 6, 9];
    let vec2 = my_vec1.iter().map(|x| format!("-{}-", x));

    for v in vec2 {
        println!("{}", v);
    }
}

fn run_into_iter() {
    let my_vec = vec![Some(1), Some(88), Some(90)];
    let vec1 = my_vec.into_iter().map(|x| x.unwrap());

    for n in vec1 {
        println!("{}", n);
    }
}

/*
    An iterator works by using a method called .next(),
    which gives an Option. When you use an iterator,
    Rust calls next() over and over again.
    If it gets Some, it keeps going. If it gets None, it stops.
*/
fn iter_next() {
    let words = vec!["this", "is", "a", "good", "day"];
    let mut my_iter = words.iter();

    // next returns refer of the value in my_iter
    assert_eq!(my_iter.next(), Some(&"this"));
    assert_eq!(my_iter.next(), Some(&"is"));
    assert_eq!(my_iter.next(), Some(&"a"));
    assert_eq!(my_iter.next(), Some(&"good"));

    /*thread 'main' panicked at 'assertion failed: `(left == right)`
     left: `Some("day")`,
    right: `Some("da")`', iterator/main.rs:53:5*/
    assert_eq!(my_iter.next(), Some(&"day"));
    assert_eq!(my_iter.next(), None);
}

fn iter_over_company() {
    let cmy = &mut ItCompany {
        name: "A SAAS startup".to_string(),
        programmers: vec![Programmer::Rust, Programmer::Go],
    };

    for programmer in cmy {
        println!("{:?}", programmer);
    }
}
