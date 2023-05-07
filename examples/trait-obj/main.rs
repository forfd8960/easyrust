trait Draw {
    fn draw(&self);
}

struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for comp in self.components.iter() {
            comp.draw();
        }
    }
}

#[derive(Debug)]
struct Button {
    weight: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw button: {:?}", self);
    }
}

#[derive(Debug)]
struct SelectBox {
    weight: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw select box: {:?}", self);
    }
}

// draw button: Button { weight: 1, height: 2, label: "Button" }
// draw select box: SelectBox { weight: 1, height: 2, options: ["opt1", "opt2"] }
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                weight: 1,
                height: 2,
                label: "Button".to_string(),
            }),
            Box::new(SelectBox {
                weight: 1,
                height: 2,
                options: vec!["opt1".to_string(), "opt2".to_string()],
            }),
        ],
    };
    screen.run();
}
