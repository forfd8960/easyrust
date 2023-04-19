use std::convert::From;

trait Concat<T> {
    type Output;
    fn concat(&self, r: T) -> Self::Output
    where
        T: ToString;
}

impl Concat<i32> for i32 {
    type Output = String;
    fn concat(&self, r: i32) -> Self::Output {
        format!("{} - {}", self.to_string(), r.to_string())
    }
}

impl Concat<&str> for &str {
    type Output = String;
    fn concat(&self, r: &str) -> Self::Output {
        format!("{} - {}", self.to_string(), r.to_string())
    }
}

#[derive(Debug)]
struct Data {
    name: String,
    desc: String,
    value: i32,
}

impl From<&str> for Data {
    fn from(value: &str) -> Self {
        let val_list: Vec<&str> = value.split(",").collect();
        Self {
            name: val_list[0].to_string(),
            desc: val_list[1].to_string(),
            value: val_list[2].parse().unwrap(),
        }
    }
}

fn main() {
    // Implement From trait for Data
    let data = Data::from("wiki,this is a wiki,90");
    println!("{:?}", data);
    println!("name: {}", data.name);
    println!("desc: {}", data.desc);
    println!("view: {}", data.value);

    // Implement Concat trait for i32 and &str
    println!("-------Implement Concat trait for i32 and &str--------");
    let my_str = "Hello";
    let my_str1 = "Dear";
    let concat_str = my_str.concat(my_str1);
    println!("concat str: {}", concat_str);

    let my_num = 999;
    let my_num2 = 688;
    let concat_num = my_num.concat(my_num2);
    println!("concated number: {}", concat_num);
}
