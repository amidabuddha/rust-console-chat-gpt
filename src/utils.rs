use colored::*;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{self, Client};
use std::io::{self, Write};

use super::models::api::{OpenAIMessage, OpenAIRequest, OpenAIResponse};
use super::models::config::ChatConfig;
use super::models::enums::{Roles, UserActions};

pub fn set_system_role(chat_config: &ChatConfig) -> String {
    return chat_config
        .chat
        .roles
        .get(&chat_config.chat.default_system_role)
        .unwrap()
        .to_string();
}

pub fn init_conversation_message(chat_config: &ChatConfig) -> OpenAIRequest {
    let system_role = set_system_role(chat_config);

    let conversation: OpenAIRequest = OpenAIRequest {
        model: chat_config.chat.model.model_name.to_string(),
        messages: vec![set_message(Roles::SYSTEM, system_role)],
    };

    return conversation;
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
                "exit" | "quit" | "bye" => Some(UserActions::EXIT),
                "flush" => Some(UserActions::FLUSH),
                "help" | "commands" => Some(UserActions::HELP),
                "save" => Some(UserActions::SAVE),
                _ => Some(UserActions::INPUT(input.to_string())),
            }
        }
        Err(_) => None,
    }
}

pub fn set_message(actor: Roles, input: String) -> OpenAIMessage {
    let message = OpenAIMessage {
        role: actor.as_str().to_string(),
        content: input,
    };

    return message;
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
