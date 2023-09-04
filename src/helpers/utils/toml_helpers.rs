use std::{fs, path::PathBuf};

use toml::Value;

pub fn open_toml(path: &PathBuf) -> String {
    fs::read_to_string(path).expect("Failed to read the file")
}

pub fn open_parse_toml(path: &PathBuf) -> Value {
    toml::from_str(&open_toml(path)).expect("Failed to parse TOML")
}

pub fn serialize_write_toml(path: &PathBuf, toml: &Value) {
    fs::write(
        path,
        toml::to_string_pretty(toml).expect("Failed to serialize TOML"),
    )
    .expect("Failed to write the file");
}
