use std::fs;
use std::path::PathBuf;

pub fn confirm_or_create(path: &PathBuf) {
    if !path.exists() {
        fs::create_dir_all(path).expect("Failed to create a directory");
    }
}
