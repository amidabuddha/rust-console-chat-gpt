pub fn flush_lines(lines: i32) {
    let escape_chars = format!("{}[F{}[K", 27 as char, 27 as char);
    print!("{}", escape_chars.repeat(lines as usize));
}
