use serde::Deserialize;
use serde_json;
use std::fs;
use std::fs::ReadDir;
use std::path::PathBuf;
use toml::Value;

pub fn confirm_or_create(path: &PathBuf) {
    if !path.exists() {
        fs::create_dir_all(path).expect("Failed to create a directory");
    }
}
pub fn read_directory(path: &PathBuf) -> ReadDir {
    fs::read_dir(path).expect("Failed to read the Directory")
}

pub fn read_file(path: &PathBuf) -> String {
    fs::read_to_string(path).expect("Failed to read the File")
}

pub fn open_parse_toml<T: for<'de> Deserialize<'de>>(path: &PathBuf) -> Result<T, ()> {
    toml::from_str(&read_file(path)).expect("Failed to parse TOML")
}

pub fn open_parse_json<T: for<'de> Deserialize<'de>>(path: &PathBuf) -> Result<T, ()> {
    serde_json::from_str(&read_file(path)).expect("Failed to parse JSON")
}

pub fn serialize_write_toml(path: &PathBuf, toml: &Value) {
    fs::write(
        path,
        toml::to_string_pretty(toml).expect("Failed to serialize TOML"),
    )
    .expect("Failed to write the file");
}
