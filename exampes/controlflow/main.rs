 fn main() {
    if_else_if();
    match_expr();
    get_num_from_match();
 }

 fn if_else_if() {
    let number = 8;
    if number == 7 {
        println!("the num is 7");
    } else if number == 8 {
        println!("num is 8");
    } else {
        println!("num is some else");
    }
 }

 fn match_expr() {
    let num = 9;
    match num {
        0 => println!("num is 0"),
        1 => println!("num is 1"),
        2 => println!("num is 2"),
        9 => println!("num is 9"),
        _ => println!("other num"),
    }
 }

 fn get_num_from_match()  {
    let input_num  = 10;
    let output_num = match input_num {
        0 => 0,
        1 => 1,
        10 => 10,
        _ => -1,
    };
    println!("output_num is: {}", output_num);
 }