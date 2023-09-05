use clearscreen::ClearScreen;
use colored::*;
use spinners::{Spinner, Spinners};
use std::env;

mod features;
use features::{
    calculate_costs::calculate_costs,
    edit_latest::edit_latest,
    format_request::format_request,
    help_info::help_info,
    load_from_file::load_from_file,
    save_chat::{check_saved, save_chat, save_chat_with_prompt},
};

mod helpers;
use helpers::{
    api_helpers::{get_openai_response, init_conversation_message, set_message},
    model_helper::select_model,
    role_helpers::role_selector,
    temperature_helpers::select_temperature,
    utils::{
        fs_helpers::{confirm_or_create, open_parse_toml_to_config},
        user_input::{flush_lines, get_user_input},
    },
};

mod models;
use models::enums::{Roles, UserActions};
use models::{api::OpenAIRequest, config::ChatConfig};

mod styling;
use styling::styling::handle_code;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let curent_exe = env::current_exe()?;
    let base_path = curent_exe.parent().unwrap();
    let config_path = &base_path.join("config.toml");
    let chat_path = &base_path.join("chats");

    // Create chats directory if it doesn't exist
    confirm_or_create(chat_path);

    // Read ChatConfig from config.toml
    let mut chat_config: ChatConfig = open_parse_toml_to_config(config_path);

    // Select model
    let model = if chat_config.chat.model_selector {
        select_model(&chat_config)
    } else {
        chat_config.chat.default_model.to_owned()
    };
    // Set API URL and API Key
    let url = format!(
        "{}{}",
        &chat_config.chat.api.base_url, &chat_config.chat.api.endpoint
    );
    let api_key = &chat_config.chat.models[&model].api_key;

    // Set chat colors
    let user_prompt_color = &chat_config.chat.colors.user_prompt;
    let assistant_prompt_color = &chat_config.chat.colors.assistant_prompt;
    let assistant_response_color = &chat_config.chat.colors.assistant_response;

    // Resume chat
    let mut conversation: OpenAIRequest;
    if let Ok(saved) = check_saved(chat_path) {
        conversation = saved;
    } else {
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
        conversation = init_conversation_message(&chat_config, &model)
    }

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
                    save_chat_with_prompt(chat_path, &conversation);
                    conversation = edit_latest(conversation, &user_prompt_color);
                }
                continue;
            }
            UserActions::EXIT => {
                if chat_config.chat.save_chat_on_exit {
                    save_chat(None, chat_path, &conversation, true);
                }
                println!("Goodbye!");
                break;
            }
            UserActions::FLUSH => {
                save_chat_with_prompt(chat_path, &conversation);
                ClearScreen::default()
                    .clear()
                    .expect("failed to clear the screen");
                conversation = init_conversation_message(&chat_config, &model);
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
                save_chat(None, chat_path, &conversation, true);
            }
            UserActions::INPUT(input) => {
                conversation.messages.push(set_message(Roles::USER, input));

                // Spinner start
                let mut sp = Spinner::new(Spinners::Dots9, "Generating Output...".into());

                let response = get_openai_response(&url, &api_key, &conversation).await?;

                // Spinner stop
                sp.stop_with_newline();
                flush_lines(1);

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
                    save_chat(
                        Some("messages".to_string()),
                        &base_path,
                        &conversation,
                        false,
                    );
                }
            }
        }
    }

    Ok(())
}
