use colored::*;
use std::path::PathBuf;
use toml::Value;

use crate::helpers::utils::fs_helpers::{
    open_parse_toml_to_config, open_parse_toml_to_value, serialize_write_toml,
};

pub fn calculate_costs(
    prompt_tokens: u64,
    completion_tokens: u64,
    prompt_price: f64,
    completion_price: f64,
) -> f64 {
    let prompt_cost = (prompt_tokens as f64) * prompt_price / 1000.0;
    let completion_cost = (completion_tokens as f64) * completion_price / 1000.0;
    prompt_cost + completion_cost
}

pub fn update_toml_file_api_usage(model: &String, path: &PathBuf, price: f64) {
    let mut toml: Value = open_parse_toml_to_value(path);
    let mut stored_price: f64 = toml["chat"]["models"][&model]["api_usage"]
        .as_float()
        .unwrap();
    stored_price += price;
    toml["chat"]["models"][&model]["api_usage"] = toml::Value::Float(stored_price);
    serialize_write_toml(path, &toml);
}

pub fn print_costs(model: &String, chat_price: f64, total_tokens: u64, path: &PathBuf) {
    let api_usage: f64 = (open_parse_toml_to_config(path)).chat.models[model].api_usage;
    println!(
        "{}",
        format!(
            "Tokens used: {}",
            total_tokens.to_string().color("green".to_string())
        )
    );
    print!("Chat cost: ");
    println!(
        "{}",
        format!("${:.2}", chat_price).color("green".to_string())
    );
    print!("Api usage cost: ");
    println!(
        "{}",
        format!("${:.2}", api_usage).color("green".to_string())
    );
}
