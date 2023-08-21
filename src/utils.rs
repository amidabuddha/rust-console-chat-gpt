use std::io::{stdin, stdout, Write};
use reqwest::{self, Client};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

use super::models::api::OpenAIRequest;
use super::models::api::OpenAIResponse;


pub fn get_user_input() -> String {
    let mut user_input_raw: String = String::new();
    print!("User: ");
    let _ = stdout().flush();
    stdin().read_line(&mut user_input_raw).expect("Failed to read input");
    let user_input = user_input_raw.trim().to_string();
    user_input
}

 pub async fn get_openai_response(url: &str, api_key: &str, conversation: &OpenAIRequest) -> OpenAIResponse
 {
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