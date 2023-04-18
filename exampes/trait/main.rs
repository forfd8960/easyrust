use std::convert::From;


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
    let data = Data::from("wiki,this is a wiki,90");
    println!("{:?}", data);
    println!("name: {}", data.name);
    println!("desc: {}", data.desc);
    println!("view: {}", data.value);
}