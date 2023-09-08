#![allow(unused)]
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Default, Deserialize, Serialize)]
pub struct ChatConfig {
    pub chat: Chat,
}

#[derive(Default, Deserialize, Serialize)]
pub struct Chat {
    pub adjust_temperature: bool,
    pub debug: bool,
    pub default_model: String,
    pub default_system_role: String,
    pub default_temperature: f64,
    pub last_completion_max_tokens: u64,
    pub model_selector: bool,
    pub role_selector: bool,
    pub save_chat_on_exit: bool,
    pub api: ChatApi,
    pub colors: ChatColors,
    pub models: BTreeMap<String, ChatModel>,
    pub roles: BTreeMap<String, String>,
}

#[derive(Default, Deserialize, Serialize)]
pub struct ChatApi {
    pub base_url: String,
    pub endpoint: String,
}

#[derive(Default, Deserialize, Serialize)]
pub struct ChatColors {
    pub user_prompt: String,
    pub assistant_prompt: String,
    pub assistant_response: String,
}

#[derive(Deserialize, Serialize)]
pub struct ChatModel {
    pub model_name: String,
    pub model_input_pricing_per_1k: f64,
    pub model_output_pricing_per_1k: f64,
    pub model_max_tokens: u64,
    pub api_key: String,
    pub api_usage: f64,
}
