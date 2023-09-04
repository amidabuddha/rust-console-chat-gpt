use std::{
    collections::BTreeMap,
    fs,
    io::{self, Write},
    path::PathBuf,
};
use toml::Value;

use super::user_input::read_user_input_no_whitespace;

pub fn flush_lines(lines: i32) {
    let escape_chars = format!("{}[F{}[K", 27 as char, 27 as char);
    print!("{}", escape_chars.repeat(lines as usize));
}

pub fn update_toml_file(
    path: &PathBuf,
    default_role: String,
    role_list: &BTreeMap<String, String>,
) {
    let mut lines = 0;
    let toml_content = fs::read_to_string(path).expect("Failed to read the file");
    let mut toml: Value = toml::from_str(&toml_content).expect("Failed to parse TOML");
    let mut role_list_toml = toml::value::Table::new();
    for (k, v) in role_list {
        role_list_toml.insert(k.to_string(), toml::Value::String(v.to_string()));
    }
    if let Some(chat) = toml.get_mut("chat").and_then(|v| v.as_table_mut()) {
        chat.insert("roles".to_string(), toml::Value::Table(role_list_toml));
    }
    loop {
        print!(
            "Would you like to have this custom role as the default role in future chats? y/n: "
        );
        lines += 1;
        io::stdout().flush().unwrap();
        let user_input = read_user_input_no_whitespace();
        if user_input.is_empty() || user_input == "n".to_string() {
            break;
        }
        if user_input == "y".to_string() {
            toml["chat"]["default_system_role"] = toml::Value::String(default_role.into());
            break;
        }
        println!("Please confirm your choice with \"y\" or \"n\"");
        lines += 1;
    }
    let updated_toml = toml::to_string_pretty(&toml).expect("Failed to serialize TOML");
    fs::write(path, updated_toml).expect("Failed to write the file");
    flush_lines(lines);
}
