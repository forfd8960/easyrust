use std::num::ParseIntError;

// question mark(?)
// "check if it is an error, and give what is inside the Result if it is okay".
// If it is not okay, it will return the error and end.
// But if it is okay, it will go to the next line
fn parse_str_to_int(num: &str) -> Result<i32, ParseIntError> {
    let number = num.parse::<i32>()?;
    Ok(number)
}

fn try_2_unwrap(nums: &Vec<Option<i32>>) {
    // thread 'main' panicked at 'first num is None', questionmark/main.rs:13:28
    println!("{}", nums[0].expect("first num is None"));
    println!("{}", nums[1].expect("second num is None"));
}

fn main() {
    let numbers = vec!["abc", "123", "456", "def"];

    /*
        Err(ParseIntError { kind: InvalidDigit })
    Ok(123)
    Ok(456)
    Err(ParseIntError { kind: InvalidDigit })
        */
    for n in numbers {
        println!("{:?}", parse_str_to_int(n));
    }

    let nums = vec![Some(88), Some(100)];
    try_2_unwrap(&nums);

    let value_list = vec!["1", "22.3", "abc"];
    let forth_str = value_list.get(3).unwrap_or(&"*");
    println!("the forth value is: {}", *forth_str);
}
