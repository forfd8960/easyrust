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
