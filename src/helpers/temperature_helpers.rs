use crate::helpers::utils::{flush_lines::flush_lines, user_input::read_user_input};

pub fn select_temperature(mut chat_temperature: f64) -> f64 {
    let mut lines = 1;
    println!("Enter a value between 0.0 and 2.0 to define GPT randomness");
    loop {
        lines += 1;
        let user_input = read_user_input(format!(
            "Press 'ENTER' for the default setting ({}): ",
            chat_temperature
        ));
        if user_input.is_empty() {
            break;
        } else {
            match user_input.replace(",", ".").parse::<f64>() {
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
    flush_lines(lines);
    chat_temperature
}
