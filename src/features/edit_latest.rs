use crate::models::api::OpenAIRequest;
use crate::models::enums::Roles;

pub fn edit_latest(mut conversation: OpenAIRequest) -> OpenAIRequest {
    if conversation.messages.last().unwrap().role == Roles::ASSISTANT.as_str() {
        conversation.messages.pop();
    }
    println!("This was the last User message in the conversation. You may rewrite it or type a new one instead:\n");
    println!("{:#?}", conversation.messages.last().unwrap().content);
    conversation.messages.pop();

    conversation
}
