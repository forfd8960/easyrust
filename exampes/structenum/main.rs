
#[derive(Debug)]
struct Date {
    year: u64,
    month: u8,
    day: u8,
}

#[derive(Debug)]
struct Project<'a> {
    name: &'a str,
    start_date: Date,
    language: Vec<&'a str>,
    code_lines: u32,
    eng_count: u8,
}

impl<'a> Project<'a> {
    fn new(name: &'a str, start_date: Date, langage: Vec<&'a str>) -> Self {
        Project {
            name: name, 
            start_date: start_date, 
            language: langage,
            code_lines: 0,
            eng_count: 0,
        }
    }

    fn increase_lines(&mut self, lines: u32) {
        self.code_lines += lines;
    }

    fn increase_eng_count(&mut self, count: u8) {
        self.eng_count += count;
    }

    fn append_lang(&mut self, new_lang: &'a str) {
        self.language.push(new_lang);
    }
}

fn main() {
    let date = Date{
        year: 2023,
        month: 4,
        day: 17,
    };
    let name = "rust-project";
    let lang = vec!["rust"];
    let prj = Project::new(&name, date, lang);
    println!("{:?}", prj);

    let date2 = Date{
        year: 2023,
        month: 4,
        day: 17,
    };
    let name = "another-project";
    let lang = vec!["rust"];
    let mut prj2 = Project::new(&name, date2, lang);
    println!("{:?}", prj2);

    prj2.append_lang("go");
    prj2.append_lang("js");
    println!("{:?}", prj2);

    prj2.increase_lines(100);
    println!("code_lines: {}", prj2.code_lines);
    prj2.increase_lines(1000);
    println!("code_lines: {}", prj2.code_lines);
}