fn take_val_by_index<'a>(container: &'a Vec<&'a str>, index: usize) -> Option<&'a str> {
    if index < container.len() {
        return Some(container[index]);
    }

    return None;
}

fn print_opt_value(index: usize) {
    let value_list = &vec!["heelo", "good", "morning"];
    let opt_val = take_val_by_index(value_list, index);

    if let Some(x) = opt_val {
        println!("{}", x);
    } else {
        println!("value is None: {:?}", opt_val);
    }
}

fn handle_option(opts: &Vec<Option<&str>>) {
    for opt in opts {
        match opt {
            Some(v) => {
                println!("option value: {}", v);
            },
            None => {
                println!("value is None");
            }
        }
    }
}

fn main() {
    /*
        morning
    value is None: None
    heelo
    value is None: None
        */
    print_opt_value(2);
    print_opt_value(3);
    print_opt_value(0);
    print_opt_value(9);

    let mut opt_vec = Vec::<Option<&str>>::new();
    let value_list = &vec!["hello", "my", "dear"];
    opt_vec.push(take_val_by_index(value_list, 0));
    opt_vec.push(take_val_by_index(value_list, 2));
    opt_vec.push(take_val_by_index(value_list, 1));
    opt_vec.push(take_val_by_index(value_list, 6));

    handle_option(&opt_vec);

    // print with iterator
    println!("print opt vec with iterator");
    opt_vec.into_iter().for_each(|v| println!("{:?}", v));
}
