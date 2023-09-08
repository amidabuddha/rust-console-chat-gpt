use std::path::PathBuf;

use crate::{
    features::save_chat::save_chat_with_prompt,
    models::{api::OpenAIRequest, config::ChatConfig},
};

pub fn exit_chat(chat_config: &ChatConfig, chat_path: &PathBuf, conversation: &OpenAIRequest) {
    if chat_config.chat.save_chat_on_exit {
        save_chat_with_prompt(chat_path, conversation)
    };
    println!("Goodbye!");
    std::process::exit(0);
}
