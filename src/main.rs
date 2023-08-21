use reqwest::{self, Client};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use std::fs;
use std::env;
use std::path::PathBuf;
use toml;

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
    let system_role: &String = config.chat.roles.get(&config.chat.default_system_role).unwrap();

    let mut conversation:OpenAIRequest = OpenAIRequest {
        model: config.chat.model.model_name,
        messages: vec![
            OpenAIMessage {
                role: "system".to_string(),
                content: system_role.to_string(),
            },
        ],
    };

    // New OpenAIMessage instance to add
    let user_message:OpenAIMessage = OpenAIMessage {
        role: "user".to_string(),
        content: "Hi!".to_string(),
    };

    // Adding the new instance to the vector
    conversation.messages.push(user_message);

    let url: String = format!("{}{}", config.chat.api.base_url, config.chat.api.endpoint);
    let api_key: String = config.chat.api.api_key;
    let client: Client = reqwest::Client::new();
    let response: OpenAIResponse = client
        .post(&url)
        .header(AUTHORIZATION, format!("Bearer {}", api_key))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .json(&conversation)
        .send()
        .await
        .expect("Failed to get response")
        .json()
        .await
        .expect("Failed to get payload");

    let choices: Vec<OpenAIResponseChoices> = response.choices;
    let assistant_message: OpenAIMessage = choices[0].message.clone();
    println!("Assistant: {}", assistant_message.content);
    conversation.messages.push(assistant_message);
    // println!("{:#?}", conversation);
}
