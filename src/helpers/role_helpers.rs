use std::{
    collections::BTreeMap,
    io::{self, Write},
    path::PathBuf,
};

use dialoguer::{theme::ColorfulTheme, Select};
use toml::Value;

use crate::{
    helpers::utils::{
        fs_helpers::{open_parse_toml_to_value, serialize_write_toml},
        user_input::{flush_lines, read_user_input, read_user_input_no_whitespace},
    },
    models::config::ChatConfig,
};

pub fn set_system_role(chat_config: &ChatConfig) -> String {
    chat_config
        .chat
        .roles
        .get(&chat_config.chat.default_system_role)
        .unwrap()
        .to_string()
}

pub fn role_selector(
    path: &PathBuf,
    mut default_role: String,
    mut role_list: BTreeMap<String, String>,
) -> (String, BTreeMap<String, String>) {
    io::stdout().flush().unwrap();
    let role_names = get_role_names(&role_list, &default_role);
    // TODO: implement preview to display role description in the selector list
    let role_name = get_selected_role(&default_role, &role_names);
    match role_names[role_name].as_str() {
        "Default" => {
            if !role_list.contains_key(&default_role) {
                println!("Default role is undefined!");
                (default_role, role_list) = custom_role_with_title(default_role, role_list);
                update_toml_file_roles(path, default_role.clone(), &role_list);
            }
        }
        "Exit" => std::process::exit(0),
        "Add new system behavior" => {
            (default_role, role_list) = custom_role(role_list);
            let mut lines = 0;
            loop {
                lines += 1;
                let user_input = read_user_input_no_whitespace(
                    "Would you like to save the custom role? y/n: ".to_string(),
                );
                if user_input.is_empty() || user_input == "n".to_string() {
                    flush_lines(lines);
                    break;
                }
                if user_input == "y".to_string() {
                    update_toml_file_roles(path, default_role.clone(), &role_list);
                    flush_lines(lines);
                    break;
                }
                println!("Please confirm your choice with \"y\" or \"n\"");
                lines += 1;
            }
        }
        _ => default_role = role_names[role_name].to_string(), // Handle other role scenarios
    }
    (default_role, role_list)
}

fn get_role_names(role_list: &BTreeMap<String, String>, default_role: &String) -> Vec<String> {
    let mut names: Vec<String> = role_list
        .iter()
        .filter(|(key, _)| !key.contains(default_role))
        .map(|(key, _)| key.clone())
        .collect();
    names.push("Add new system behavior".to_string());
    names.push("Exit".to_string());
    names.insert(0, "Default".to_string());
    names
}

fn get_selected_role(default_role: &str, role_names: &[String]) -> usize {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "Select a role or use the default one \"{}\"",
            default_role
        ))
        .default(0)
        .items(role_names)
        .interact()
        .unwrap()
}

fn custom_role(role_list: BTreeMap<String, String>) -> (String, BTreeMap<String, String>) {
    let mut lines = 0;
    loop {
        lines += 1;
        let custom_role =
            read_user_input_no_whitespace("Enter a title for the new role: ".to_string());
        if custom_role.is_empty() {
            println!("Please fill the title!");
            lines += 1;
            continue;
        }
        if role_list.contains_key(&custom_role) {
            println!("Such role name already exists!");
            lines += 1;
            continue;
        }
        flush_lines(lines);
        return custom_role_with_title(custom_role, role_list);
    }
}

fn custom_role_with_title(
    role_name: String,
    mut role_list: BTreeMap<String, String>,
) -> (String, BTreeMap<String, String>) {
    let mut lines = 0;
    loop {
        lines += 1;
        let user_input =
            read_user_input("Enter a detailed description of your assistant role: ".to_string());
        if user_input.is_empty() {
            println!("Please fill the description!");
            lines += 1;
            continue;
        }
        flush_lines(lines);
        role_list.insert(role_name.to_string(), user_input);
        return (role_name, role_list);
    }
}

fn update_toml_file_roles(
    path: &PathBuf,
    default_role: String,
    role_list: &BTreeMap<String, String>,
) {
    let mut toml: Value = open_parse_toml_to_value(path);
    update_roles_for_config(default_role, role_list, &mut toml);
    serialize_write_toml(path, &toml);
}

fn update_roles_for_config(
    default_role: String,
    role_list: &BTreeMap<String, String>,
    toml: &mut Value,
) {
    let mut lines = 0;
    let mut role_list_toml = toml::value::Table::new();
    for (k, v) in role_list {
        role_list_toml.insert(k.to_string(), toml::Value::String(v.to_string()));
    }
    if let Some(chat) = toml.get_mut("chat").and_then(|v| v.as_table_mut()) {
        chat.insert("roles".to_string(), toml::Value::Table(role_list_toml));
    }
    if !(toml["chat"]["default_system_role"] == toml::Value::String(default_role.to_owned())) {
        loop {
            lines += 1;
            let user_input = read_user_input_no_whitespace(
            "Would you like to have this custom role as the default role in future chats? y/n: "
                .to_string(),
        );
            if user_input.is_empty() || user_input == "n".to_string() {
                break;
            }
            if user_input == "y".to_string() {
                toml["chat"]["default_system_role"] = toml::Value::String(default_role.into());
                break;
            }
            println!("Please confirm your choice with \"y\" or \"n\"");
            lines += 1;
        }
    }
    flush_lines(lines);
}
