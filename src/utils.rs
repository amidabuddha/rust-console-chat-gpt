use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};
use regex::Regex;
use reqwest::{
    self,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use std::{
    collections::BTreeMap,
    fs,
    io::{self, Write},
    path::PathBuf,
};
use toml::Value;

use super::models::api::{OpenAIMessage, OpenAIRequest, OpenAIResponse};
use super::models::config::ChatConfig;
use super::models::enums::{Roles, UserActions};

pub fn set_system_role(chat_config: &ChatConfig) -> String {
    chat_config
        .chat
        .roles
        .get(&chat_config.chat.default_system_role)
        .unwrap()
        .to_string()
}

pub fn init_conversation_message(chat_config: &ChatConfig) -> OpenAIRequest {
    let system_role = set_system_role(chat_config);

    OpenAIRequest {
        model: chat_config.chat.model.model_name.to_string(),
        temperature: chat_config.chat.temperature,
        messages: vec![set_message(Roles::SYSTEM, system_role)],
    }
}

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

pub fn set_message(actor: Roles, input: String) -> OpenAIMessage {
    OpenAIMessage {
        role: actor.as_str().to_string(),
        content: input,
    }
}

pub async fn get_openai_response(
    url: &str,
    api_key: &str,
    conversation: &OpenAIRequest,
) -> Result<OpenAIResponse, reqwest::Error> {
    let client: Client = reqwest::Client::new();
    let response: OpenAIResponse = client
        .post(url)
        .header(AUTHORIZATION, format!("Bearer {}", api_key))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .json(&conversation)
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}

fn flush_lines(lines: i32) {
    let escape_chars = format!("{}[F{}[K", 27 as char, 27 as char);
    print!("{}", escape_chars.repeat(lines as usize));
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

pub fn role_selector(
    path: &PathBuf,
    mut default_role: String,
    mut role_list: BTreeMap<String, String>,
) -> (String, BTreeMap<String, String>) {
    let role_names = get_role_names(&role_list);
    // TODO: implement preview to display role description in the selector list
    let role_name = get_selected_role(&default_role, &role_names);
    flush_lines(1);
    match role_names[role_name].as_str() {
        "Default" => {}
        "Exit" => std::process::exit(0),
        "Add new system behavior" => {
            (default_role, role_list) = custom_role(role_list);
            let mut lines = 0;
            loop {
                print!("Would you like to save the custom role? y/n: ");
                lines += 1;
                io::stdout().flush().unwrap();
                let user_input = read_user_input_no_whitespace();
                if user_input.is_empty() || user_input == "n".to_string() {
                    flush_lines(lines);
                    break;
                }
                if user_input == "y".to_string() {
                    update_toml_file(path, default_role.clone(), &role_list);
                    flush_lines(lines);
                    break;
                }
                println!("Please confirm your choice with \"y\" or \"n\"");
                lines += 1;
            }
        }
        _ => default_role = role_names[role_name].to_string(), // Handle other role scenarios
    }
    (default_role, role_list)
}

fn get_role_names(role_list: &BTreeMap<String, String>) -> Vec<String> {
    let mut names: Vec<String> = role_list
        .iter()
        .filter(|(key, _)| !key.contains("dev"))
        .map(|(key, _)| key.clone())
        .collect();
    names.push("Add new system behavior".to_string());
    names.push("Exit".to_string());
    names.insert(0, "Default".to_string());
    names
}

fn get_selected_role(default_role: &str, role_names: &[String]) -> usize {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "Select a role or use the default one \"{}\"",
            default_role
        ))
        .default(0)
        .items(role_names)
        .interact()
        .unwrap()
}

fn custom_role(mut role_list: BTreeMap<String, String>) -> (String, BTreeMap<String, String>) {
    let mut lines = 0;
    loop {
        print!("Enter a title for the new role: ");
        lines += 1;
        io::stdout().flush().unwrap();
        let user_input = read_user_input_no_whitespace();
        if user_input.is_empty() {
            println!("Please fill the title!");
            lines += 1;
            continue;
        }
        if role_list.contains_key(&user_input) {
            println!("Such role name already exists!");
            lines += 1;
            continue;
        }
        let custom_role = user_input;
        loop {
            print!("Enter a detailed description of your assistant role: ");
            lines += 1;
            io::stdout().flush().unwrap();
            let user_input = read_user_input();
            if user_input.is_empty() {
                println!("Please fill the description!");
                lines += 1;
                continue;
            }
            flush_lines(lines);
            role_list.insert(custom_role.to_string(), user_input);
            return (custom_role, role_list);
        }
    }
}

fn read_user_input() -> String {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    user_input.trim().to_string()
}
fn read_user_input_no_whitespace() -> String {
    let user_input = read_user_input();
    let re = Regex::new(r"[ \t\n]+").unwrap();
    re.replace_all(&user_input, "_")
        .replace("\\n", "")
        .to_string()
}

fn update_toml_file(path: &PathBuf, default_role: String, role_list: &BTreeMap<String, String>) {
    let mut lines = 0;
    let toml_content = fs::read_to_string(path).expect("Failed to read the file");
    let mut toml: Value = toml::from_str(&toml_content).expect("Failed to parse TOML");
    let mut role_list_toml = toml::value::Table::new();
    for (k, v) in role_list {
        role_list_toml.insert(k.to_string(), toml::Value::String(v.to_string()));
    }
    if let Some(chat) = toml.get_mut("chat").and_then(|v| v.as_table_mut()) {
        chat.insert("roles".to_string(), toml::Value::Table(role_list_toml));
    }
    loop {
        print!(
            "Would you like to have this custom role as the default role in future chats? y/n: "
        );
        lines += 1;
        io::stdout().flush().unwrap();
        let user_input = read_user_input_no_whitespace();
        if user_input.is_empty() || user_input == "n".to_string() {
            break;
        }
        if user_input == "y".to_string() {
            toml["chat"]["default_system_role"] = toml::Value::String(default_role.into());
            break;
        }
        println!("Please confirm your choice with \"y\" or \"n\"");
        lines += 1;
    }
    let updated_toml = toml::to_string_pretty(&toml).expect("Failed to serialize TOML");
    fs::write(path, updated_toml).expect("Failed to write the file");
    flush_lines(lines);
}
