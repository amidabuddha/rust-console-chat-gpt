use std::env;
use std::fs;
use toml;

mod utils;
use utils::{get_openai_response, get_user_input, save_chat};

mod models {
    pub mod api;
    pub mod config;
    pub mod enums;
}
use models::api::{OpenAIMessage, OpenAIRequest};
use models::config::ChatConfig;
use models::enums::UserAction;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_path = env::current_dir()?;
    let config_path = base_path.join("config.toml");
    let chat_path = base_path.join("chats");
    if !chat_path.exists() {
        fs::create_dir_all(&chat_path)?;
    }
    let toml_str = fs::read_to_string(config_path)?;
    let chat_config: ChatConfig = toml::from_str(&toml_str)?;
    let url = format!("{}{}", chat_config.chat.api.base_url, chat_config.chat.api.endpoint);
    let api_key = chat_config.chat.api.api_key;

    // implement temperature_selector
    // implement role_selector
    let system_role = chat_config
        .chat
        .roles
        .get(&chat_config.chat.default_system_role)
        .unwrap();

    let mut conversation: OpenAIRequest = OpenAIRequest {
        model: chat_config.chat.model.model_name,
        messages: vec![OpenAIMessage {
            role: "system".to_string(),
            content: system_role.to_string(),
        }],
    };

    while let Some(user_input) = get_user_input() {
        match user_input {
            /*
            implement cost
            implement edit
            implement file
            implement format
            */
            UserAction::NONE => {
                println!("Please enter your message!");
                continue;
            }
            UserAction::EXIT => {
                println!("Goodbye!");
                if chat_config.chat.save_chat_on_exit {
                    save_chat("".to_string(), &chat_path, &conversation);
                };
                break;
            }
            UserAction::FLUSH => {
                println!("Are you sure?");
                continue;
            }
            UserAction::HELP => {
                println!("Are you sure?");
                continue;
            }
            UserAction::SAVE => {
                save_chat("".to_string(), &chat_path, &conversation);
            }
            UserAction::INPUT(input) => {
                let user_message = OpenAIMessage {
                    role: "user".to_string(),
                    content: input,
                };
                conversation.messages.push(user_message);

                let response = get_openai_response(&url, &api_key, &conversation).await?;

                let choices = response.choices;
                let assistant_message = choices[0].message.clone();
                println!("Assistant: {}", assistant_message.content);
                conversation.messages.push(assistant_message);
                if chat_config.chat.debug {
                    // println!("{:#?}", conversation);
                    save_chat("messages.json".to_string(), &base_path, &conversation);
                }
            }
        }
    }

    Ok(())
}
