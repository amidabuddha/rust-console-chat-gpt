#![allow(unused)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct OpenAIMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAIRequest {
    pub model: String,
    pub messages: Vec<OpenAIMessage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAIResponseChoices {
    index: usize,
    pub message: OpenAIMessage,
    finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAIResponseUsage {
    prompt_tokens: usize,
    completion_tokens: usize,
    total_tokens: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAIResponse {
    id: String,
    object: String,
    created: usize,
    model: String,
    pub choices: Vec<OpenAIResponseChoices>,
    pub usage: OpenAIResponseUsage,
}