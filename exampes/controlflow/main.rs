 fn main() {
    if_else_if();
    match_expr();
    get_num_from_match();
    match_tuple();
    match_guard();
    match_part_value((12, 0, 0));
    match_part_value((9, 59, 0));
    match_part_value((23, 59, 0));
    match_part_value((10, 0, 59));
    match_part_value((10, 6, 30));
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

 fn match_tuple() {
    let sky = "cloudy";
    let temp = "warm";
    match (sky, temp) {
        ("cloudy", "warm") => println!("oh warm cloudy day"),
        ("clear", "warm") => println!("nice day"),
        _ => println!("what day is it today"),
    }
 }

 fn match_guard() {
    let day = 30;
    let lines = 1000;
    match (day, lines) {
        (day, lines) => { 
            if day == 30 || day == 31 {
                println!("oh seems last day")
            }
            if lines > 900 {
                println!("productive day");
            }
        },
    }
 }

 fn match_part_value(time: (u8, u8, u8)) {
    match time {
        (hour, _, _,) if hour == 12 => {println!("time for lunch")},
        (hour, minute, _,) if hour == 23 && minute == 59 => {
            println!("another day gone");
        },
        (_, minute, _) if minute == 59 => println!("another hour gone"),
        (_, _, second) if second == 59 => println!("another minute gone"),
        t => {
            println!("{:?}", t);
        }
    }
 }