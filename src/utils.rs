use chrono::prelude::*;
use colored::Colorize;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{self, Client};
use serde_json::to_string_pretty;
use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;

use super::models::api::OpenAIRequest;
use super::models::api::OpenAIResponse;

pub fn get_user_input(user_prompt_color: &str) -> String {
    let mut user_input_raw: String = String::new();
    print!("{}", "User: ".color(user_prompt_color.to_string()));
    let _ = stdout().flush();
    stdin()
        .read_line(&mut user_input_raw)
        .expect("Failed to read input");
    let user_input = user_input_raw.trim().to_string();
    user_input
}

pub async fn get_openai_response(
    url: &str,
    api_key: &str,
    conversation: &OpenAIRequest,
) -> OpenAIResponse {
    let client: Client = reqwest::Client::new();
    let response: OpenAIResponse = client
        .post(url)
        .header(AUTHORIZATION, format!("Bearer {}", api_key))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .json(&conversation)
        .send()
        .await
        .expect("Failed to get response")
        .json()
        .await
        .expect("Failed to get payload");

    response
}

pub fn save_chat(name: String, path: &PathBuf, conversation: &OpenAIRequest) {
    let timestamp = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let file_name = if name == "" {
        format!("{timestamp}{}", ".json".to_string())
    } else {
        name
    };
    let json = to_string_pretty(&conversation).expect("Serialization failed");
    let mut file = File::create(path.join(&file_name)).expect("File creation failed");
    file.write_all(json.as_bytes()).expect("Write failed");
    println!("{} saved to {:?}", &file_name, &path);
}
