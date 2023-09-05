use crate::models::api::OpenAIRequest;
use crate::models::enums::Roles;
use colored::*;

pub fn edit_latest(mut conversation: OpenAIRequest, user_prompt_color: &String) -> OpenAIRequest {
    if conversation.messages.last().unwrap().role == Roles::ASSISTANT.as_str() {
        conversation.messages.pop();
    }
    println!("This was the last User message in the conversation. You may copy and rewrite it or type a new one instead:");
    println!(
        "{} {}",
        "[User]".color(user_prompt_color.to_string()),
        conversation.messages.last().unwrap().content
    );
    conversation.messages.pop();

    conversation
}
