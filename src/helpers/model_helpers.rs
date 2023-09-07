use crate::models::config::ChatConfig;
use dialoguer::{theme::ColorfulTheme, Select};

pub fn select_model(config: &ChatConfig) -> String {
    let mut gpt_models: Vec<(String, String)> = Vec::new();
    for (key, value) in &config.chat.models {
        gpt_models.push((key.to_string(), value.model_name.to_string()))
    }
    let default_value = gpt_models
        .iter()
        .position(|(first, _)| first == &config.chat.default_model.to_string())
        .unwrap_or_else(|| 0);
    let model_names: Vec<String> = gpt_models.iter().map(|model| model.1.clone()).collect();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the preferred GPT model:")
        .default(default_value)
        .items(&model_names)
        .interact()
        .unwrap_or_default();
    gpt_models[selection].0.to_string()
}
