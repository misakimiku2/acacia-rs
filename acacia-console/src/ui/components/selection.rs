#[derive(Clone, Copy)]
pub struct TextSelection {
    pub anchor: usize,
    pub focus: usize,
}

impl TextSelection {
    pub fn new(index: usize) -> Self {
        Self { anchor: index, focus: index }
    }

    pub fn range(&self) -> (usize, usize) {
        if self.anchor <= self.focus { (self.anchor, self.focus) } else { (self.focus, self.anchor) }
    }

    pub fn copy(&self, text: &str) -> Option<String> {
        let (start, end) = self.range();
        if start == end {
            return None;
        }
        Some(text.chars().skip(start).take(end - start).collect())
    }
}

pub fn char_index_at_display_column(text: &str, target_column: usize) -> usize {
    let mut column = 0usize;
    for (index, ch) in text.chars().enumerate() {
        let ch_width = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0).max(1);
        if target_column <= column.saturating_add(ch_width / 2) {
            return index;
        }
        column += ch_width;
        if target_column < column {
            return index + 1;
        }
    }

    text.chars().count()
}
