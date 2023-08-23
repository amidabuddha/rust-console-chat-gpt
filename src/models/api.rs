#![allow(unused)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct OpenAIMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OpenAIRequest {
    pub model: String,
    pub messages: Vec<OpenAIMessage>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OpenAIResponseChoices {
    index: u64,
    pub message: OpenAIMessage,
    finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OpenAIResponseUsage {
    prompt_tokens: u64,
    completion_tokens: u64,
    total_tokens: u64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OpenAIResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    pub choices: Vec<OpenAIResponseChoices>,
    pub usage: OpenAIResponseUsage,
}