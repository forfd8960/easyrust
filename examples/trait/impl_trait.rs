use std::fmt::Display;

fn larger_data<T: PartialOrd + Display>(one: T, two: T) {
    let large_data = if one >= two { one } else { two };

    println!("{} is larger", large_data);
}

fn print_data(data: impl Into<String> + Display) {
    println!("input data: {}", data);
}

fn get_fn(instr: &str) -> impl FnMut(i32) -> i32 {
    match instr {
        "double" => |mut item| {
            item *= 2;
            println!(" double {}", item);
            item
        },
        "triple" => |mut item| {
            item *= 3;
            println!("triple {}", item);
            item
        },
        _ => |item| {
            println!("do nothing for: {}", item);
            item
        },
    }
}

/*
10 is larger
def is larger
abc is larger
input data: hello, world!
input data: this is good day
 double 12
double 6: 12
triple 18
triple 6: 18
do nothing for: 6
nothing for 6: 6
*/
fn main() {
    larger_data(8, 10);
    larger_data("abc", "def");
    larger_data("abc", "ABC");

    print_data("hello, world!");
    print_data("this is good day".to_string());

    println!("double 6: {}", get_fn("double")(6));
    println!("triple 6: {}", get_fn("triple")(6));
    println!("nothing for 6: {}", get_fn("")(6));
}
