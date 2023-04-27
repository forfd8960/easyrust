#[derive(Debug)]
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
    lang_list: Vec<Lang>,
}

impl LangList {
    fn push(&mut self, lang: Lang) {
        self.lang_list.push(lang)
    }

    fn list_langs(&self, result: &mut Vec<String>) {
        for l in &self.lang_list {
            result.push(l.to_string())
        }
    }
}

#[derive(Debug)]
struct Project<'a> {
    name: &'a str,
    start_date: &'a Date,
    language: &'a mut LangList,
    code_lines: u32,
    eng_count: u8,
}

impl<'a> Project<'a> {
    fn new(name: &'a str, start_date: &'a Date, langages: &'a mut LangList) -> Self {
        Project {
            name: name,
            start_date: start_date,
            language: langages,
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

    fn append_lang(&mut self, new_lang: Lang) {
        self.language.push(new_lang);
    }

    fn list_langs(&self, result: &mut Vec<String>) {
        self.language.list_langs(result)
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
        lang_list: vec![Lang::Rust],
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

    let result = &mut Vec::<String>::new();
    prj2.list_langs(result);
    println!("{:?}", result);
}
