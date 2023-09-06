use crate::helpers::utils::user_input::{read_multiline, read_user_input};

pub fn format_request() -> String {
    println!("Paste the multiline text and press 'Ctrl+D' on an new empty line to continue:");
    let mut content = read_multiline();
    let context = read_user_input(
        "Add additional clarification before the formatted text or press 'ENTER' to continue: "
            .to_string(),
    );
    if !context.is_empty() {
        content = context + ":\n" + &content;
    }
    content
}
