use std::io::{self, Write};

use colored::*;
use regex::Regex;

use crate::models::enums::UserActions;

pub fn get_user_input(user_prompt_color: &String) -> Option<UserActions> {
    print!("{} ", "User:".color(user_prompt_color.to_string()));
    let user_input = read_user_input("".to_string()).to_lowercase();
    match user_input.as_str() {
        "" => Some(UserActions::NONE),
        "cost" => Some(UserActions::COST),
        "edit" => Some(UserActions::EDIT),
        "exit" | "quit" | "bye" => Some(UserActions::EXIT),
        "file" => Some(UserActions::FILE),
        "flush" => Some(UserActions::FLUSH),
        "format" => Some(UserActions::FORMAT),
        "help" | "commands" => Some(UserActions::HELP),
        "save" => Some(UserActions::SAVE),
        _ => Some(UserActions::INPUT(user_input.to_string())),
    }
}

pub fn read_user_input(prompt: String) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    user_input.trim().to_string()
}

pub fn read_user_input_no_whitespace(prompt: String) -> String {
    let user_input = read_user_input(prompt);
    let re = Regex::new(r"[ \t\n]+").unwrap();
    re.replace_all(&user_input, "_")
        .replace("\\n", "")
        .to_string()
}
