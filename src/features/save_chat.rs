use chrono::prelude::*;
use serde_json;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::helpers::utils::flush_lines::flush_lines;
use crate::helpers::utils::user_input::read_user_input_no_whitespace;
use crate::models::api::OpenAIRequest;

// pub fn check_saved(path: &PathBuf) -> OpenAIRequest {

//     // Check if there are files in chat folder
//     let entries: Vec<_> = fs::read_dir(path)?.collect();
//     if entries.is_empty() {
//         return Ok(());
//     }

//     // Get file names in a list
//     let mut file_names = entries.into_iter().filter_map(|entry| {
//         match entry {
//             Ok(entry) => entry.file_name().into_string().ok(),
//             Err(_) => None,
//         }
//     }).collect::<Vec<_>>();

//     // Use dialoguer to create a selection of file names
//     file_names.insert(0, "Skip".to_string());
//     file_names.push("Exit".to_string());

//     let selection = Select::new()
//         .items(&file_names)
//         .default(0)
//         .interact()?;

//     match file_names[selection].as_str() {
//         "Exit" => std::process::exit(0),
//         "Skip" => return Ok(()),
//         file_name => {
//             let mut file = fs::File::open(format!("{}{}", path, file_name))?;
//             let mut contents = String::new();
//             file.read_to_string(&mut contents)?;

//             // Read as JSON content the selected file into an object of type OpenAIRequest
//             // (Assuming OpenAIRequest is serde-serializable)
//             let _open_ai_req: OpenAIRequest = serde_json::from_str(&contents)?;

//             // Add actions to be done with open_ai_req object here...
//         }
//     }

//     Ok(())
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
