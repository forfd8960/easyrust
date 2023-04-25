#[derive(Debug)]
struct MySchedule {
    event: String,
    date: String,
    start_at: String,
    end_at: String,
}

impl From<&str> for MySchedule {
    fn from(value: &str) -> Self {
        let data: Vec<&str> = value.split(",").into_iter().collect();
        Self {
            event: data[0].to_string(),
            date: data[1].to_string(),
            start_at: data[2].to_string(),
            end_at: data[3].to_string(),
        }
    }
}

fn main() {
    let my_schedule = "get_up,tmr,7AM,7:10AM";
    let sched = MySchedule::from(my_schedule);
    // MySchedule { event: "get_up", date: "tmr", start_at: "7AM", end_at: "7:10AM" }
    println!("{:?}", sched);
}
