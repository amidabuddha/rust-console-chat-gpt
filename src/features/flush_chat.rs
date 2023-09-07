use std::path::PathBuf;

use crate::{
    helpers::api_helpers::init_conversation_message,
    models::{api::OpenAIRequest, config::ChatConfig},
};
use clearscreen::ClearScreen;

use super::save_chat::save_chat_with_prompt;

pub fn flush_chat(
    chat_config: &ChatConfig,
    model: &String,
    chat_path: &PathBuf,
    conversation: OpenAIRequest,
) -> OpenAIRequest {
    save_chat_with_prompt(chat_path, &conversation);
    ClearScreen::default()
        .clear()
        .expect("failed to clear the screen");
    init_conversation_message(&chat_config, &model)
}
