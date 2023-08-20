#![allow(unused)]
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ChatConfig {
    pub chat: Chat,
}

#[derive(Debug, Deserialize)]
pub struct Chat {
    temperature: usize,
    adjust_temperature: bool,
    default_system_role: String,
    role_selector: bool,
    save_chat_on_exit: bool,
    debug: bool,
    last_completion_max_tokens: usize,
    pub api: ChatApi,
    colors: ChatColors,
    pub model: ChatModel,
    pub roles: Vec<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct ChatApi {
    pub api_key: String,
    pub base_url: String,
    pub endpoint: String,
    api_usage: f64,
}

#[derive(Debug, Deserialize)]
struct ChatColors {
    code: String,
    user_prompt: String,
    assistant_prompt: String,
    assistant_response: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatModel {
    pub model_name: String,
    model_input_pricing_per_1k: f64,
    model_output_pricing_per_1k: f64,
    model_max_tokens: usize,
}