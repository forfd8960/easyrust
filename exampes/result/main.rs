#[derive(Debug)]
enum MyError {
    NumTooLarge(String),
    NumTooSmall(String),
}

fn get_result(n: i32) -> Result<i32, MyError> {
    if n <= 100 && n >= 0 {
        return Ok(n);
    }

    if n > 100 {
        let s = format!("{} is too large", n);
        return Err(MyError::NumTooLarge(s));
    }

    let s = format!("{} is too small", n);
    return Err(MyError::NumTooSmall(s));
}

fn main() {
    let res = get_result(100);
    println!("{:?}", res);
    let res = get_result(-1);
    println!("{:?}", res);
    let res = get_result(1000);
    println!("{:?}", res);

    if let Ok(v) = get_result(100) {
        println!("ok value: {}", v);
    }

    if let Err(err) = get_result(1000) {
        match err {
            MyError::NumTooLarge(err_msg) => println!("{}", err_msg),
            MyError::NumTooSmall(err_msg) => println!("{}", err_msg),
        }
    }
}
