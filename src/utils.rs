use chrono::prelude::*;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{self, Client};
use serde_json;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

use super::models::api::{OpenAIRequest, OpenAIResponse};
use super::models::enums::UserAction;

pub fn get_user_input() ->  Option<UserAction> {
    print!("User: ");
    let mut user_input = String::new();
    io::stdout().flush().unwrap();
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            let input = user_input.trim().to_lowercase();
            match input.as_str() {
                "" => Some(UserAction::NONE),
                "exit" | "quit" | "bye" => Some(UserAction::EXIT),
                "flush" => Some(UserAction::FLUSH),
                "help" | "command" => Some(UserAction::HELP),
                "save" => Some(UserAction::SAVE),
                _ => Some(UserAction::INPUT(input.to_string())),
            }
        }
        Err(_) => None,
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

pub fn save_chat(name: String, path: &PathBuf, conversation: &OpenAIRequest) {
    let timestamp = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let file_name = if name == "" {
        format!("{timestamp}{}", ".json".to_string())
    } else {
        name
    };
    let json = serde_json::to_string_pretty(&conversation).expect("Serialization failed");
    let mut file = File::create(path.join(&file_name)).expect("File creation failed");
    file.write_all(json.as_bytes()).expect("Write failed");
    println!("{} saved to {:?}", &file_name, &path);
}
