use std::env::current_dir;
use std::fs::{create_dir_all, read_to_string};
use std::path::PathBuf;
use toml::from_str;
use colored::Colorize;

mod utils;
use utils::get_user_input;
use utils::get_openai_response;
use utils::save_chat;

mod models {
    pub mod api;
    pub mod config;
}

use models::api::OpenAIMessage;
use models::api::OpenAIRequest;
use models::api::OpenAIResponse;
use models::api::OpenAIResponseChoices;
use models::config::ChatConfig;

#[tokio::main]
async fn main() {
    let base_path: PathBuf = current_dir().unwrap();
    let config_path: PathBuf = base_path.join("config.toml");
    let chat_path: PathBuf = base_path.join("chats");
    if !chat_path.exists() {
        create_dir_all(&chat_path).unwrap();
    }
    let toml_str: String = read_to_string(config_path).expect("Failed to read config file");
    let config: ChatConfig = from_str(&toml_str).expect("Failed to deserialize config.toml");
    let url: String = format!("{}{}", config.chat.api.base_url, config.chat.api.endpoint);
    let api_key: String = config.chat.api.api_key;
    let default_role: String = config.chat.default_system_role;
    let user_label_color: String = config.chat.colors.user_prompt;
    let assistant_label_color: String = config.chat.colors.assistant_prompt;
    let assistant_prompt_color: String = config.chat.colors.assistant_response;
    let save_on_exit: bool = config.chat.save_chat_on_exit;
    let debug: bool = config.chat.debug;

    // implement temperature_selector
    // implement role_selector
    let system_role: &String = config
        .chat
        .roles
        .get(&default_role)
        .unwrap();

    let mut conversation: OpenAIRequest = OpenAIRequest {
        model: config.chat.model.model_name,
        messages: vec![OpenAIMessage {
            role: "system".to_string(),
            content: system_role.to_string(),
        }],
    };
    loop {
        let user_input: String = get_user_input(&user_label_color);
        match user_input.as_str() {
            "" => {
                println!("Please enter your message!");
                continue;
            }
            "exit" => {
                println!("Goodbye!");
                if save_on_exit {
                    save_chat("".to_string(), &chat_path, &conversation);
                }
                break;
            },
            "save" => {
                save_chat("".to_string(), &chat_path, &conversation);
                return;
            },
            "flush" => {println!("Are you sure?"); String::new()},
            "help" | "command" => {println!("Are you sure?"); String::new()}
            _ => user_input.to_string()
        };
        /*
        implement cost
        implement edit
        implement file
        implement format
        implement save
        */

        let user_message: OpenAIMessage = OpenAIMessage {
            role: "user".to_string(),
            content: user_input.to_string(),
        };

        conversation.messages.push(user_message);

        let response: OpenAIResponse = get_openai_response(&url, &api_key, &conversation).await;

        let choices: Vec<OpenAIResponseChoices> = response.choices;
        let assistant_message: OpenAIMessage = choices[0].message.clone();
        println!("{} {}", "Assistant: ".color(assistant_label_color.to_string()), assistant_message.content.color(assistant_prompt_color.to_string()));
        conversation.messages.push(assistant_message);
        if debug {
            // println!("{:#?}", conversation);
            save_chat("messages.json".to_string(), &base_path, &conversation);
        }
    }
}
