#![allow(unused)]
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
pub struct ChatConfig {
    pub chat: Chat,
}

#[derive(Debug, Deserialize, Default)]
pub struct Chat {
    pub temperature: f64,
    pub adjust_temperature: bool,
    pub default_system_role: String,
    pub role_selector: bool,
    pub save_chat_on_exit: bool,
    pub debug: bool,
    pub last_completion_max_tokens: u64,
    pub api: ChatApi,
    pub colors: ChatColors,
    pub model: ChatModel,
    pub roles: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct ChatApi {
    pub api_key: String,
    pub base_url: String,
    pub endpoint: String,
    pub api_usage: f64,
}

#[derive(Debug, Deserialize, Default)]
pub struct ChatColors {
    pub user_prompt: String,
    pub assistant_prompt: String,
    pub assistant_response: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct ChatModel {
    pub model_name: String,
    model_input_pricing_per_1k: f64,
    model_output_pricing_per_1k: f64,
    model_max_tokens: u64,
}
