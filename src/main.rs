use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use std::fs;
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
    // TODO change this to be %appdata%
    let base_path = "/Users/stefan/Documents/Playground/OpenAI/rust-console-chat-gpt";
    let config_path = format!("{}/config.toml", base_path);
    let toml_str = fs::read_to_string(config_path).expect("Failed to read config file");
    let config: ChatConfig = toml::from_str(&toml_str).expect("Failed to deserialize config.toml");

    let system_role = config.chat.roles
        .iter()
        .find(|role| role.contains_key("dev"))
        .unwrap()
        .get("dev")
        .unwrap();

    let mut conversation = OpenAIRequest {
        model: config.chat.model.model_name,
        messages: vec![
            OpenAIMessage {
                role: "system".to_string(),
                content: system_role.to_string(),
            },
        ],
    };

    // New OpenAIMessage instance to add
    let user_message = OpenAIMessage {
        role: "user".to_string(),
        content: "Hi!".to_string(),
    };

    // Adding the new instance to the vector
    conversation.messages.push(user_message);

    let url = format!("{}{}", config.chat.api.base_url, config.chat.api.endpoint);
    let api_key = config.chat.api.api_key;
    let client = reqwest::Client::new();
    let response: OpenAIResponse = client
        .post(&url)
        .header(AUTHORIZATION, format!("Bearer {}", api_key))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .json(&conversation)
        .send()
        .await
        .expect("Failed to get response:")
        .json()
        .await
        .expect("Failed to get payload:");

    let choices: Vec<OpenAIResponseChoices> = response.choices;
    let message = &choices[0].message;
    println!("Assistant: {}", message.content);
}
