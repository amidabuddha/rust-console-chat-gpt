use colored::*;
use regex::Regex;

pub fn handle_code(content: String, content_color: String) {
    let re = Regex::new(r"```.*?\n").unwrap();
    let content_no_lang = re.replace_all(&content, "```");
    let fragments: Vec<&str> = content_no_lang.split("```").collect();
    for (index, fragment) in fragments.iter().enumerate() {
        if index % 2 == 0 {
            println!("{}", fragment.color(content_color.to_string()));
        } else {
            println!("{}", fragment);
        }
    }
}

// TODO: add code syntax highlighting
