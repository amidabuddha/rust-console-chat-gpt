#![allow(unused)]
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ChatConfig {
    pub chat: Chat,
}

#[derive(Debug, Deserialize)]
pub struct Chat {
    pub temperature: usize,
    pub adjust_temperature: bool,
    pub default_system_role: String,
    pub role_selector: bool,
    pub save_chat_on_exit: bool,
    pub debug: bool,
    pub last_completion_max_tokens: usize,
    pub api: ChatApi,
    pub colors: ChatColors,
    pub model: ChatModel,
    pub roles: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct ChatApi {
    pub api_key: String,
    pub base_url: String,
    pub endpoint: String,
    api_usage: f64,
}

#[derive(Debug, Deserialize)]
pub struct ChatColors {
    code: String,
    pub user_prompt: String,
    pub assistant_prompt: String,
    pub assistant_response: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatModel {
    pub model_name: String,
    model_input_pricing_per_1k: f64,
    model_output_pricing_per_1k: f64,
    model_max_tokens: usize,
}