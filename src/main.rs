use std::env;
use std::fs;
use std::path::PathBuf;
use toml;

mod utils;
use utils::get_user_input;
use utils::get_openai_response;

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
    let base_path: PathBuf = env::current_dir().unwrap();
    let config_path: PathBuf = base_path.join("config.toml");
    let toml_str: String = fs::read_to_string(config_path).expect("Failed to read config file");
    let config: ChatConfig = toml::from_str(&toml_str).expect("Failed to deserialize config.toml");
    let url: String = format!("{}{}", config.chat.api.base_url, config.chat.api.endpoint);
    let api_key: String = config.chat.api.api_key;

    // implement temperature_selector
    // implement role_selector
    let system_role: &String = config
        .chat
        .roles
        .get(&config.chat.default_system_role)
        .unwrap();

    let mut conversation: OpenAIRequest = OpenAIRequest {
        model: config.chat.model.model_name,
        messages: vec![OpenAIMessage {
            role: "system".to_string(),
            content: system_role.to_string(),
        }],
    };
    loop {
        let user_input: String = get_user_input();
        match user_input.as_str() {
            "exit" => {
                println!("Goodbye!");
                if config.chat.save_chat_on_exit{
                    //save funtion
                    println!("To be saved...")
                }
                break;
            },
            "flush" => {println!("Are you sure?"); String::new()},
            "help" => {println!("Are you sure?"); String::new()}
            _ => user_input.to_string()
        };
        /*
        implement cost
        implement edit
        implement exit
        implement file
        implement flush
        implement format
        implement save
        implement help | commands
        */

        let user_message: OpenAIMessage = OpenAIMessage {
            role: "user".to_string(),
            content: user_input.to_string(),
        };

        conversation.messages.push(user_message);

        let response: OpenAIResponse = get_openai_response(&url, &api_key, &conversation).await;

        let choices: Vec<OpenAIResponseChoices> = response.choices;
        let assistant_message: OpenAIMessage = choices[0].message.clone();
        println!("Assistant: {}", assistant_message.content);
        conversation.messages.push(assistant_message);
        // println!("{:#?}", conversation);
    }
}
