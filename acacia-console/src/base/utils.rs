pub fn strip_ansi(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' && chars.peek() == Some(&'[') {
            chars.next();
            for sequence_char in chars.by_ref() {
                if ('@'..='~').contains(&sequence_char) {
                    break;
                }
            }
        } else {
            result.push(ch);
        }
    }

    result
}
