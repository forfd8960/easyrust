trait Formatter {
    fn format(&self, input: &mut String) -> String;
}

struct MDFormat;
impl Formatter for MDFormat {
    fn format(&self, input: &mut String) -> String {
        format!("markdown formatted: {}", input)
    }
}

struct HtmlFormat;
impl Formatter for HtmlFormat {
    fn format(&self, input: &mut String) -> String {
        format!("html formatted: {}", input)
    }
}

struct TomlFormat;
impl Formatter for TomlFormat {
    fn format(&self, input: &mut String) -> String {
        format!("toml formatted: {}", input)
    }
}

// Trait object must include dyn keyword
fn format_it(input: &mut String, fmts: Vec<&dyn Formatter>) {
    for fmt in fmts {
        println!("{}", fmt.format(input));
    }
}

fn main() {
    let mut text = "Hello, Dear".to_string();
    let html_format: &dyn Formatter = &HtmlFormat;
    let md_format: &dyn Formatter = &MDFormat;
    let fmts = vec![html_format, md_format];
    format_it(&mut text, fmts);
}
