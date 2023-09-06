use crate::helpers::utils::user_input::{add_context, read_multiline};

pub fn format_request() -> String {
    println!("Paste the multiline text and press 'Ctrl+D' on an new empty line to continue:");
    let mut content = read_multiline();
    content = add_context(content);
    content
}
