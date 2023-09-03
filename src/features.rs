use chrono::prelude::*;
use serde_json;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

use super::models::api::OpenAIRequest;
use super::models::enums::{Roles, UserActions};

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
    let commands: [UserActions; 6] = [
        // UserActions::COST,
        UserActions::EDIT,
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

pub fn edit_latest(mut conversation: OpenAIRequest) -> OpenAIRequest {
    if conversation.messages.last().unwrap().role == Roles::ASSISTANT.as_str() {
        conversation.messages.pop();
    }
    println!("This was the last User message in the conversation. You may rewrite it or type a new one instead:\n");
    println!("{:#?}", conversation.messages.last().unwrap().content);
    conversation.messages.pop();

    conversation
}

fn flush_lines(lines: i32) {
    let escape_chars = format!("{}[F{}[K", 27 as char, 27 as char);
    print!("{}", escape_chars.repeat(lines as usize));
    print!("{}", 27 as char);
}

pub fn select_temperature(mut chat_temperature: f64) -> f64 {
    let mut lines = 1;
    println!("Enter a value between 0.0 and 2.0 to define GPT randomness");
    loop {
        print!(
            "Press 'ENTER' for the default setting ({}): ",
            chat_temperature
        );
        lines += 1;
        io::stdout().flush().unwrap();
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                if user_input.trim().is_empty() {
                    break;
                } else {
                    match user_input.trim().replace(",", ".").parse::<f64>() {
                        Ok(value) => {
                            if value >= 0.0 && value <= 2.0 {
                                chat_temperature = value;
                                break;
                            } else {
                                println!("Invalid input. Please enter a floating-point number between 0.0 and 2.0.");
                                lines += 1;
                                continue;
                            }
                        }
                        Err(_) => {
                            println!("Invalid input. Please enter a valid floating-point number.");
                            lines += 1;
                            continue;
                        }
                    }
                }
            }
            Err(_) => {
                println!("Failed to read input.");
                lines += 1;
                continue;
            }
        }
    }
    flush_lines(lines);
    chat_temperature
}
