use clearscreen::ClearScreen;
use std::env;

mod features;
mod styling;
use features::{
    calculate_costs::print_costs,
    edit_latest::edit_latest,
    format_request::format_request,
    help_info::help_info,
    load_from_file::load_from_file,
    save_chat::{check_saved, save_chat, save_chat_with_prompt},
};

mod helpers;
use helpers::{
    api_helpers::{chat_completion, init_conversation_message},
    model_helper::select_model,
    role_helpers::role_selector,
    temperature_helpers::select_temperature,
    utils::{
        fs_helpers::{confirm_or_create, open_parse_toml_to_config, prompt_file_path},
        user_input::get_user_input,
    },
};

mod models;
use models::enums::UserActions;
use models::{api::OpenAIRequest, config::ChatConfig};

#[tokio::main]
pub async fn chat() -> Result<(), Box<dyn std::error::Error>> {
    let curent_exe = env::current_exe()?;
    let base_path = curent_exe.parent().unwrap();
    let config_path = if base_path.join("config.toml").exists() {
        base_path.join("config.toml")
    } else {
        println!("\"config.toml\" not found in the current directory!");
        prompt_file_path()
    };
    let chat_path = &base_path.join("chats");

    // Create chats directory if it doesn't exist
    confirm_or_create(chat_path);

    // Read ChatConfig from config.toml
    let mut chat_config: ChatConfig = open_parse_toml_to_config(&config_path);

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

    // Set model price and current chat costs
    let prompt_price = &chat_config.chat.models[&model].model_input_pricing_per_1k;
    let completion_price = &chat_config.chat.models[&model].model_output_pricing_per_1k;
    let mut chat_price = 0.0;
    let mut total_tokens = 0;

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
                &config_path,
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
                print_costs(&model, chat_price, total_tokens, &config_path);
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
                (conversation, chat_price, total_tokens) = chat_completion(
                    &chat_config,
                    chat_path,
                    &config_path,
                    base_path,
                    conversation,
                    format_request(),
                    &url,
                    api_key,
                    assistant_prompt_color,
                    assistant_response_color,
                    &model,
                    *prompt_price,
                    *completion_price,
                    chat_price,
                    total_tokens,
                )
                .await
                .unwrap();
            }
            UserActions::FILE => {
                let user_message = load_from_file();
                if user_message.is_empty() {
                    continue;
                } else {
                    (conversation, chat_price, total_tokens) = chat_completion(
                        &chat_config,
                        chat_path,
                        &config_path,
                        base_path,
                        conversation,
                        user_message,
                        &url,
                        api_key,
                        assistant_prompt_color,
                        assistant_response_color,
                        &model,
                        *prompt_price,
                        *completion_price,
                        chat_price,
                        total_tokens,
                    )
                    .await
                    .unwrap();
                }
            }
            UserActions::HELP | UserActions::COMMANDS => {
                help_info();
                continue;
            }
            UserActions::SAVE => {
                save_chat(None, chat_path, &conversation, true);
            }
            UserActions::INPUT(input) => {
                (conversation, chat_price, total_tokens) = chat_completion(
                    &chat_config,
                    chat_path,
                    &config_path,
                    base_path,
                    conversation,
                    input,
                    &url,
                    api_key,
                    assistant_prompt_color,
                    assistant_response_color,
                    &model,
                    *prompt_price,
                    *completion_price,
                    chat_price,
                    total_tokens,
                )
                .await
                .unwrap();
            }
        }
    }
    Ok(())
}
