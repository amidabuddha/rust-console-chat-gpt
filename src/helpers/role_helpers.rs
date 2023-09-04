use std::{
    collections::BTreeMap,
    io::{self, Write},
    path::PathBuf,
};

use dialoguer::{theme::ColorfulTheme, Select};

use crate::helpers::utils::sysutils::{flush_lines, update_toml_file};
use crate::helpers::utils::user_input::{read_user_input, read_user_input_no_whitespace};
use crate::models::config::ChatConfig;

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
    let role_names = get_role_names(&role_list);
    // TODO: implement preview to display role description in the selector list
    let role_name = get_selected_role(&default_role, &role_names);
    flush_lines(1);
    match role_names[role_name].as_str() {
        "Default" => {}
        "Exit" => std::process::exit(0),
        "Add new system behavior" => {
            (default_role, role_list) = custom_role(role_list);
            let mut lines = 0;
            loop {
                print!("Would you like to save the custom role? y/n: ");
                lines += 1;
                io::stdout().flush().unwrap();
                let user_input = read_user_input_no_whitespace();
                if user_input.is_empty() || user_input == "n".to_string() {
                    flush_lines(lines);
                    break;
                }
                if user_input == "y".to_string() {
                    update_toml_file(path, default_role.clone(), &role_list);
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

fn get_role_names(role_list: &BTreeMap<String, String>) -> Vec<String> {
    let mut names: Vec<String> = role_list
        .iter()
        .filter(|(key, _)| !key.contains("dev"))
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

fn custom_role(mut role_list: BTreeMap<String, String>) -> (String, BTreeMap<String, String>) {
    let mut lines = 0;
    loop {
        print!("Enter a title for the new role: ");
        lines += 1;
        io::stdout().flush().unwrap();
        let user_input = read_user_input_no_whitespace();
        if user_input.is_empty() {
            println!("Please fill the title!");
            lines += 1;
            continue;
        }
        if role_list.contains_key(&user_input) {
            println!("Such role name already exists!");
            lines += 1;
            continue;
        }
        let custom_role = user_input;
        loop {
            print!("Enter a detailed description of your assistant role: ");
            lines += 1;
            io::stdout().flush().unwrap();
            let user_input = read_user_input();
            if user_input.is_empty() {
                println!("Please fill the description!");
                lines += 1;
                continue;
            }
            flush_lines(lines);
            role_list.insert(custom_role.to_string(), user_input);
            return (custom_role, role_list);
        }
    }
}
