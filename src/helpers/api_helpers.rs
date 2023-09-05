use reqwest::{
    self,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Client,
};

use crate::models::api::{OpenAIMessage, OpenAIRequest, OpenAIResponse};
use crate::models::config::ChatConfig;
use crate::models::enums::Roles;

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
