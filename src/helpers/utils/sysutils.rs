use std::{fs, path::PathBuf};

use toml::Value;

pub fn flush_lines(lines: i32) {
    let escape_chars = format!("{}[F{}[K", 27 as char, 27 as char);
    print!("{}", escape_chars.repeat(lines as usize));
}

pub fn open_parse_toml(path: &PathBuf) -> Value {
    toml::from_str(&fs::read_to_string(path).expect("Failed to read the file"))
        .expect("Failed to parse TOML")
}

pub fn serialize_write_toml(path: &PathBuf, toml: &Value) {
    fs::write(
        path,
        toml::to_string_pretty(toml).expect("Failed to serialize TOML"),
    )
    .expect("Failed to write the file");
}
