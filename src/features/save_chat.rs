use chrono::prelude::*;
use serde_json;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::helpers::utils::flush_lines::flush_lines;
use crate::helpers::utils::user_input::read_user_input_no_whitespace;
use crate::models::api::OpenAIRequest;

// pub fn check_saved() {
//     // TODO: check folder chats for previous chats and ask
//     // 1. check if chat folder exist
//     // 2. check if there are files in chat folder
//     // 3. get file names in a list
//     // 4. use dialoguer to crate a selection of file names
//     // 4.1 adding "Skip" as top selection that continue to the next code block without any action
//     // 4.2 appending "Exit" as last selection that implements this match: "Exit" => std::process::exit(0),
//     // 5. read as JSON content the selected file into an object of type OpenAIRequest
// }

pub fn save_chat(
    mut custom_file_name: Option<String>,
    path: &PathBuf,
    conversation: &OpenAIRequest,
    ask: bool,
) {
    let mut file_name = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    if ask {
        custom_file_name = Some(read_user_input_no_whitespace(
            "Name the JSON file to save the chat or hit 'Enter' for default name: ".to_string(),
        ));
    }
    if let Some(name) = custom_file_name.as_ref().map(|s| s.as_str()) {
        if name != "" {
            file_name = name.strip_suffix(".json").unwrap_or(name).to_string();
        }
    }
    let json = serde_json::to_string_pretty(&conversation).expect("Serialization failed");
    let mut file = File::create(path.join(format!("{}{}", file_name, ".json".to_string())))
        .expect("File creation failed");
    file.write_all(json.as_bytes()).expect("Write failed");
    println!("{}.json saved to {:?}", file_name, &path);
}

pub fn save_chat_with_prompt(path: &PathBuf, conversation: &OpenAIRequest) {
    let mut lines = 0;
    loop {
        lines += 1;
        let user_input =
            read_user_input_no_whitespace("Would you like to save this chat? y/n: ".to_string());
        if user_input.is_empty() || user_input == "n".to_string() {
            break;
        }
        if user_input == "y".to_string() {
            flush_lines(lines);
            lines = 0;
            save_chat(None, path, conversation, true);
            break;
        }
        println!("Please confirm your choice with \"y\" or \"n\"");
        lines += 1;
    }
    flush_lines(lines);
}
