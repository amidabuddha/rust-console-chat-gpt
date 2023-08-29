use colored::*;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

pub fn handle_code(content: String, content_color: String) {
    let fragments = parse_code_blocks(&content);
    for fragment in fragments {
        if fragment.0 == "" {
            println!("{}", fragment.1.color(content_color.to_string()));
        } else {
            code_coloring(&fragment.1, &fragment.0);
        }
    }
}

fn code_coloring(code: &str, lang: &str) {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps
        .find_syntax_by_name(lang)
        .unwrap_or_else(|| ps.find_syntax_plain_text());
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    for line in LinesWithEndings::from(&code) {
        // LinesWithEndings enables use of newlines mode
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
        print!("{}", escaped);
    }
}

fn parse_code_blocks(input: &str) -> Vec<(String, String)> {
    let mut result = Vec::new();
    let mut current_lang = "";
    let mut current_code = String::new();
    let mut in_code_block = false;
    for line in input.lines() {
        if line.starts_with("```") {
            if in_code_block {
                result.push((capitalize(current_lang), current_code));
                current_lang = "";
                current_code = "".to_string();
                in_code_block = false;
            } else {
                current_lang = &line[3..];
                current_code = "".to_string();
                in_code_block = true;
            }
        } else if in_code_block {
            current_code += line;
            current_code += "\n";
        } else {
            result.push(("".to_string(), line.to_string()));
        }
    }
    if in_code_block {
        result.push((capitalize(current_lang), current_code));
    }
    result
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
