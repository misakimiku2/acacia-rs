use unicode_width::UnicodeWidthChar;

pub fn fit_text_to_width(text: &str, width: usize) -> String {
    let mut result = String::new();
    let mut used_width = 0usize;

    for ch in text.chars() {
        let ch_width = ch.width().unwrap_or(0).max(1);
        if used_width.saturating_add(ch_width) > width {
            break;
        }
        result.push(ch);
        used_width += ch_width;
    }

    result.push_str(&" ".repeat(width.saturating_sub(used_width)));
    result
}

pub fn wrap_text_by_width(text: &str, width: usize) -> Vec<String> {
    let width = width.max(1);
    let mut lines = Vec::new();
    for segment in text.split('\n') {
        let mut current = String::new();
        let mut current_width = 0usize;
        for ch in segment.chars() {
            let ch_width = ch.width().unwrap_or(0).max(1);
            if current_width > 0 && current_width.saturating_add(ch_width) > width {
                lines.push(current);
                current = String::new();
                current_width = 0;
            }
            current.push(ch);
            current_width += ch_width;
        }
        lines.push(current);
    }
    lines
}
