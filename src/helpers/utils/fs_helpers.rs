// use dialoguer::Input;
use serde_json;
use std::fs;
use std::fs::ReadDir;
use std::path::PathBuf;
use toml::Value;

use crate::models::api::OpenAIRequest;
use crate::models::config::ChatConfig;

use super::user_input::read_user_input_no_whitespace;

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

pub fn open_parse_toml_to_config(path: &PathBuf) -> ChatConfig {
    toml::from_str(&read_file(path)).expect("Failed to parse TOML")
}

pub fn open_parse_toml_to_value(path: &PathBuf) -> Value {
    toml::from_str(&read_file(path)).expect("Failed to parse TOML")
}

pub fn open_parse_json(path: &PathBuf) -> OpenAIRequest {
    serde_json::from_str(&read_file(path)).expect("Failed to parse JSON")
}

pub fn serialize_write_toml(path: &PathBuf, toml: &Value) {
    fs::write(
        path,
        toml::to_string(toml).expect("Failed to serialize TOML"),
    )
    .expect("Failed to write the file");
}

pub fn prompt_file_path() -> PathBuf {
    let file_path = read_user_input_no_whitespace(
        "Please enter the file name /with a path if not in the current directory/: ".to_string(),
    );
    PathBuf::from(file_path)
}
