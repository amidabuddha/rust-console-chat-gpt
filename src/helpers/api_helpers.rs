use std::path::Path;

use colored::Colorize;
use reqwest::{
    self,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use spinners::{Spinner, Spinners};

use crate::models::{
    api::{OpenAIMessage, OpenAIRequest, OpenAIResponse},
    config::ChatConfig,
    enums::Roles,
};
use crate::{
    features::save_chat::save_chat, helpers::utils::user_input::flush_lines,
    styling::styling::handle_code,
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
    base_path: &Path,
    mut conversation: OpenAIRequest,
    user_message: String,
    url: &String,
    api_key: &String,
    assistant_prompt_color: &String,
    assistant_response_color: &String,
) -> Result<OpenAIRequest, reqwest::Error> {
    conversation
        .messages
        .push(set_message(Roles::USER, user_message));

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
    Ok(conversation)
}
