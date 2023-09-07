use std::path::{Path, PathBuf};

use colored::Colorize;
use reqwest::{
    self,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use spinners::{Spinner, Spinners};

use crate::{
    features::save_chat::save_chat, helpers::utils::user_input::flush_lines,
    styling::styling::handle_code,
};
use crate::{
    features::{
        calculate_costs::{calculate_costs, update_toml_file_api_usage},
        save_chat::save_chat_with_prompt,
    },
    models::{
        api::{OpenAIMessage, OpenAIRequest, OpenAIResponse},
        config::ChatConfig,
        enums::Roles,
    },
};

use super::role_helpers::set_system_role;

pub fn init_conversation_message(chat_config: &ChatConfig, model: &String) -> OpenAIRequest {
    let system_role = set_system_role(chat_config);

    OpenAIRequest {
        model: chat_config.chat.models[model].model_name.to_string(),
        temperature: chat_config.chat.temperature,
        messages: vec![set_message(Roles::SYSTEM, system_role)],
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

pub async fn chat_completion(
    chat_config: &ChatConfig,
    chat_path: &PathBuf,
    config_path: &PathBuf,
    base_path: &Path,
    mut conversation: OpenAIRequest,
    user_message: String,
    url: &String,
    api_key: &String,
    assistant_prompt_color: &String,
    assistant_response_color: &String,
    model: &String,
    prompt_price: f64,
    completion_price: f64,
    mut chat_price: f64,
    mut total_tokens: u64,
) -> Result<(OpenAIRequest, f64, u64), reqwest::Error> {
    conversation
        .messages
        .push(set_message(Roles::USER, user_message));

    // Spinner start
    let mut sp = Spinner::new(Spinners::Dots9, "Generating Output...".into());

    let response = match get_openai_response(&url, &api_key, &conversation).await {
        Ok(response) => response,
        Err(err) => {
            sp.stop_with_newline();
            flush_lines(1);
            println!("Something went wrong...");
            save_chat_with_prompt(chat_path, &conversation);
            println!("{}", err);
            std::process::exit(1)
        }
    };

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
    let usage = response.usage;
    total_tokens += usage.total_tokens;
    let chat_iteration_price = calculate_costs(
        usage.prompt_tokens,
        usage.completion_tokens,
        prompt_price,
        completion_price,
    );
    update_toml_file_api_usage(model, config_path, chat_iteration_price);
    chat_price += chat_iteration_price;
    if chat_config.chat.debug {
        save_chat(
            Some("messages".to_string()),
            &base_path,
            &conversation,
            false,
        );
    }
    Ok((conversation, chat_price, total_tokens))
}
