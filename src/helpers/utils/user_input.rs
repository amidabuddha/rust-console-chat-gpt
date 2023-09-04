use std::io::{self, Write};

use colored::*;
use regex::Regex;

use crate::models::enums::UserActions;

pub fn get_user_input(user_prompt_color: &String) -> Option<UserActions> {
    print!("{} ", "User:".color(user_prompt_color.to_string()));
    let mut user_input = String::new();
    io::stdout().flush().unwrap();
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            let input = user_input.trim().to_lowercase();
            match input.as_str() {
                "" => Some(UserActions::NONE),
                "cost" => Some(UserActions::COST),
                "edit" => Some(UserActions::EDIT),
                "exit" | "quit" | "bye" => Some(UserActions::EXIT),
                "file" => Some(UserActions::FILE),
                "flush" => Some(UserActions::FLUSH),
                "format" => Some(UserActions::FORMAT),
                "help" | "commands" => Some(UserActions::HELP),
                "save" => Some(UserActions::SAVE),
                _ => Some(UserActions::INPUT(input.to_string())),
            }
        }
        Err(_) => None,
    }
}

pub fn read_user_input() -> String {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    user_input.trim().to_string()
}

pub fn read_user_input_no_whitespace() -> String {
    let user_input = read_user_input();
    let re = Regex::new(r"[ \t\n]+").unwrap();
    re.replace_all(&user_input, "_")
        .replace("\\n", "")
        .to_string()
}
