use chrono::prelude::*;
use serde_json;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use super::models::api::OpenAIRequest;
use super::models::enums::UserActions;

pub fn save_chat(name: String, path: &PathBuf, conversation: &OpenAIRequest) {
    // TODO: implement ask and skip_exit
    let timestamp = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let file_name = if name == "" {
        format!("{timestamp}{}", ".json".to_string())
    } else {
        name
    };
    let json = serde_json::to_string_pretty(&conversation).expect("Serialization failed");
    let mut file = File::create(path.join(&file_name)).expect("File creation failed");
    file.write_all(json.as_bytes()).expect("Write failed");
    println!("{} saved to {:?}", &file_name, &path);
}

pub fn help_info() {
    /*
    Prints the available commands
    */
    println!("You can use the following commands:");
    let commands: [UserActions; 5] = [
        // UserActions::COST,
        // UserActions::EDIT,
        UserActions::EXIT,
        // UserActions::FILE,
        UserActions::FLUSH,
        // UserActions::FORMAT,
        UserActions::SAVE,
        UserActions::HELP,
        UserActions::COMMANDS,
    ];
    for command in commands {
        println!("{}", command.description());
    }
}
