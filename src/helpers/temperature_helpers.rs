use crate::helpers::utils::sysutils::flush_lines;
use std::io::{self, Write};

pub fn select_temperature(mut chat_temperature: f64) -> f64 {
    let mut lines = 1;
    println!("Enter a value between 0.0 and 2.0 to define GPT randomness");
    loop {
        print!(
            "Press 'ENTER' for the default setting ({}): ",
            chat_temperature
        );
        lines += 1;
        io::stdout().flush().unwrap();
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                if user_input.trim().is_empty() {
                    break;
                } else {
                    match user_input.trim().replace(",", ".").parse::<f64>() {
                        Ok(value) => {
                            if value >= 0.0 && value <= 2.0 {
                                chat_temperature = value;
                                break;
                            } else {
                                println!("Invalid input. Please enter a floating-point number between 0.0 and 2.0.");
                                lines += 1;
                                continue;
                            }
                        }
                        Err(_) => {
                            println!("Invalid input. Please enter a valid floating-point number.");
                            lines += 1;
                            continue;
                        }
                    }
                }
            }
            Err(_) => {
                println!("Failed to read input.");
                lines += 1;
                continue;
            }
        }
    }
    flush_lines(lines);
    chat_temperature
}
