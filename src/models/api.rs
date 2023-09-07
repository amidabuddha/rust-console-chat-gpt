#![allow(unused)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OpenAIMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Serialize)]
pub struct OpenAIRequest {
    pub model: String,
    pub temperature: f64,
    pub messages: Vec<OpenAIMessage>,
}

#[derive(Debug, Deserialize)]
pub struct OpenAIResponseChoices {
    index: u64,
    pub message: OpenAIMessage,
    finish_reason: String,
}

#[derive(Debug, Deserialize)]
pub struct OpenAIResponseUsage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

#[derive(Debug, Deserialize)]
pub struct OpenAIResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    pub choices: Vec<OpenAIResponseChoices>,
    pub usage: OpenAIResponseUsage,
}
