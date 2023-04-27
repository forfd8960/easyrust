struct HHKB {
    short_cuts: Vec<String>,
}

impl Keyboard for HHKB {
    fn type_words(&self) {
        println!("wrting with HHKB: {:?}", self.short_cuts);
    }
}

struct Falcon {
    short_cuts: Vec<String>,
}

impl Keyboard for Falcon {
    fn type_words(&self) {
        println!("wrting with Falcon: {:?}", self.short_cuts);
    }
}

trait Keyboard {
    fn type_words(&self);
}

fn main() {
    let mut keyboards: Vec<Box<&dyn Keyboard>> = Vec::new();

    let short_cuts = "delete".to_string();
    let short_cuts1 = "fn + del".to_string();
    let kb1 = HHKB {
        short_cuts: vec![short_cuts, short_cuts1],
    };

    let short_cuts3 = "delete".to_string();
    let kb2 = Falcon {
        short_cuts: vec![short_cuts3],
    };

    let kb1_dyn: &dyn Keyboard = &kb1;
    let kb2_dyn: &dyn Keyboard = &kb2;
    keyboards.push(Box::new(kb1_dyn));
    keyboards.push(Box::new(kb2_dyn));

    for kb in keyboards {
        kb.as_ref().type_words();
    }
}
