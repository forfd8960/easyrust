use std::rc::Rc;

/*
An Rc is like a good office worker: Rc writes down who has ownership, and how many.
Then once the number of owners goes down to 0, the variable can disappear.
*/

enum Kind {
    k1(Rc<String>),
    k2(Rc<String>),
}

impl Kind {
    fn to_string(&self) -> Rc<String> {
        match &self {
            Kind::k1(v) => v.clone(),
            Kind::k2(v) => v.clone(),
        }
    }

    fn strong_count(&self) -> usize {
        match &self {
            Kind::k1(v) => Rc::strong_count(&v),
            Kind::k2(v) => Rc::strong_count(&v),
        }
    }
}

struct Fruit<'a> {
    name: &'a str,
    kind: Kind,
}

fn main() {
    let f1 = Fruit {
        name: "apple",
        kind: Kind::k1(Rc::new("apple-1".to_string())),
    };

    let f2 = Fruit {
        name: "orange",
        kind: Kind::k1(Rc::new("orange-1".to_string())),
    };

    let fruits: Vec<Rc<String>> = vec![f1.kind.to_string(), f2.kind.to_string()];
    for f_kind in fruits {
        println!("{}", f_kind);
        println!("count: {}", Rc::strong_count(&f_kind));
    }

    println!(
        "kind: {}, count: {}",
        f1.kind.to_string(),
        f1.kind.strong_count(),
    );
    println!(
        "kind: {}, count: {}",
        f2.kind.to_string(),
        f2.kind.strong_count(),
    );
}
