use std::cell::RefCell;

#[derive(Debug, Clone)]
enum Lang {
    Rust,
    Go,
    Java,
    TypeScript,
}

impl Lang {
    fn to_string(&self) -> String {
        match self {
            Lang::Rust => "with Rust".to_string(),
            Lang::Go => "with Golang".to_string(),
            Lang::Java => "with Java".to_string(),
            Lang::TypeScript => "with TS".to_string(),
        }
    }
}

#[derive(Debug)]
struct Date {
    year: u64,
    month: u8,
    day: u8,
}

#[derive(Debug)]
struct LangList {
    lang_list: RefCell<Vec<Lang>>,
}

impl LangList {
    fn push(&self, lang: Lang) {
        self.lang_list.borrow_mut().push(lang)
    }
}

#[derive(Debug)]
struct Project<'a> {
    name: &'a str,
    start_date: &'a Date,
    language: &'a LangList,
    code_lines: u32,
    eng_count: u8,
}

impl<'a> Project<'a> {
    fn new(name: &'a str, start_date: &'a Date, langs: &'a LangList) -> Self {
        Project {
            name: name,
            start_date: start_date,
            language: langs,
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

    fn append_lang(&self, new_lang: Lang) {
        self.language.push(new_lang);
    }
}

fn main() {
    let date = Date {
        year: 2023,
        month: 4,
        day: 17,
    };
    let name = "rust-project";
    let lang = &mut LangList {
        lang_list: RefCell::new(vec![Lang::Rust]),
    };
    let prj = Project::new(&name, &date, lang);
    println!("{:?}", prj);

    let name = "another-project";
    let mut prj2 = Project::new(&name, &date, lang);
    println!("{:?}", prj2);

    prj2.append_lang(Lang::Go);
    prj2.append_lang(Lang::TypeScript);
    prj2.append_lang(Lang::Java);
    println!("{:?}", prj2);

    prj2.increase_lines(100);
    println!("code_lines: {}", prj2.code_lines);
    prj2.increase_lines(1000);
    println!("code_lines: {}", prj2.code_lines);

    let lang_list: Vec<Lang> = prj2.language.lang_list.borrow().to_vec();
    for lang in lang_list {
        println!("programming-language: {:?}", lang);
    }
}
