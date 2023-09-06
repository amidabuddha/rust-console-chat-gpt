use crate::helpers::utils::{
    fs_helpers::{prompt_file_path, read_file},
    user_input::add_context,
};

pub fn load_from_file() -> String {
    let mut content = "".to_string();
    let path = prompt_file_path();
    if !path.exists() {
        println!("Missing file specified, back to chat...");
    } else {
        content = add_context(read_file(&path));
    }
    content
}
