use colored::*;
use std::env;
use std::fs;
use toml;

mod features;
use features::features::{
    calculate_costs, edit_latest, format_request, help_info, load_from_file, save_chat,
};

mod helpers;
use helpers::api_helpers::{get_openai_response, init_conversation_message, set_message};
use helpers::role_helpers::role_selector;
use helpers::temperature_helpers::select_temperature;
use helpers::utils::user_input::get_user_input;

mod models;
use models::config::ChatConfig;
use models::enums::{Roles, UserActions};

mod styling;
use styling::styling::handle_code;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_path = env::current_dir()?;
    let config_path = &base_path.join("config.toml");
    let chat_path = base_path.join("chats");

    // Create chats directory if it doesn't exist
    if !chat_path.exists() {
        fs::create_dir_all(&chat_path)?;
    }

    // Read ChatConfig from config.toml
    let toml_str = fs::read_to_string(config_path)?;
    let mut chat_config: ChatConfig = toml::from_str(&toml_str)?;

    // Set API URL and API Key
    let url = format!(
        "{}{}",
        &chat_config.chat.api.base_url, &chat_config.chat.api.endpoint
    );
    let api_key = &chat_config.chat.api.api_key;

    // Set chat colors
    let user_prompt_color = &chat_config.chat.colors.user_prompt;
    let assistant_prompt_color = &chat_config.chat.colors.assistant_prompt;
    let assistant_response_color = &chat_config.chat.colors.assistant_response;

    // Set chat temperature
    if chat_config.chat.adjust_temperature {
        chat_config.chat.temperature = select_temperature(chat_config.chat.temperature);
    }

    // Set custom role
    if chat_config.chat.role_selector {
        let (default_role, roles) = role_selector(
            config_path,
            chat_config.chat.default_system_role,
            chat_config.chat.roles,
        );
        chat_config.chat.default_system_role = default_role;
        chat_config.chat.roles = roles;
    }

    let mut conversation = init_conversation_message(&chat_config);

    while let Some(user_input) = get_user_input(&user_prompt_color) {
        match user_input {
            UserActions::NONE => {
                println!("Please enter your message!");
                continue;
            }
            UserActions::COST => {
                // TODO: implement
                calculate_costs();
                continue;
            }
            UserActions::EDIT => {
                if conversation.messages.len() < 2 {
                    println!("Seems like your chat has not started yet...");
                } else {
                    save_chat("".to_string(), &chat_path, &conversation);
                    conversation = edit_latest(conversation);
                }
                continue;
            }
            UserActions::EXIT => {
                println!("Goodbye!");
                if chat_config.chat.save_chat_on_exit {
                    save_chat("".to_string(), &chat_path, &conversation);
                }
                break;
            }
            UserActions::FLUSH => {
                if chat_config.chat.save_chat_on_exit {
                    save_chat("".to_string(), &chat_path, &conversation);
                }
                conversation = init_conversation_message(&chat_config);
                continue;
            }
            UserActions::FORMAT => {
                // TODO: implement
                format_request();
                continue;
            }
            UserActions::FILE => {
                // TODO: implement
                load_from_file();
                continue;
            }
            UserActions::HELP | UserActions::COMMANDS => {
                help_info();
                continue;
            }
            UserActions::SAVE => {
                save_chat("".to_string(), &chat_path, &conversation);
            }
            UserActions::INPUT(input) => {
                conversation.messages.push(set_message(Roles::USER, input));

                let response = get_openai_response(&url, &api_key, &conversation).await?;

                let choices = response.choices;
                let assistant_message = &choices[0].message;
                print!(
                    "{} ",
                    "Assistant:".color(assistant_prompt_color.to_string())
                );
                handle_code(
                    assistant_message.content.to_string(),
                    assistant_response_color.to_string(),
                );
                conversation.messages.push(assistant_message.to_owned());
                if chat_config.chat.debug {
                    save_chat("messages.json".to_string(), &base_path, &conversation);
                }
            }
        }
    }

    Ok(())
}
