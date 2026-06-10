use arboard::Clipboard;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
};
use std::time::Duration;
use unicode_width::UnicodeWidthChar;

use crate::{
    base::time_tick::TimeTick,
    ui::components::{selection::TextSelection, style},
};

pub struct InputLayout {
    pub text: String,
    pub selection: Option<TextSelection>,
    pub cursor: usize,
    pub cursor_visible: bool,
    pub cursor_following: bool,
    pub scroll: u16,

    cursor_blink_tick: TimeTick,
    cursor_visible_status: bool,
}

impl InputLayout {
    const MOUSE_SCROLL_LINES: i32 = 3;

    pub fn new() -> Self {
        Self {
            text: String::new(),
            selection: None,
            cursor: 0,
            cursor_visible: true,
            cursor_following: true,
            scroll: 0,
            cursor_blink_tick: TimeTick::new(Duration::from_millis(500)),
            cursor_visible_status: false,
        }
    }

    // Render

    pub fn render(&mut self, area: Rect, has_popup: bool) -> Vec<Line<'static>> {
        self.update_cursot_status(has_popup);
        let text_color = style::normal_text_color(has_popup);
        let vertical_scroll = self.vertical_scroll(area.width, area.height);
        let input_width = area.width.max(1) as usize;
        let selection = self.selection.map(|selection| selection.range()).filter(|(start, end)| start != end);

        let mut content_lines = Vec::new();
        let mut current_spans = Vec::new();
        let mut current_line_len = 0usize;

        for (index, ch) in self.text.chars().enumerate() {
            let ch_width = ch.width().unwrap_or(0).max(1);

            if current_line_len > 0 && current_line_len.saturating_add(ch_width) > input_width {
                content_lines.push(Line::from(current_spans));
                current_spans = Vec::new();
                current_line_len = 0;
            }

            let item_style = if self.cursor_visible_status && index == self.cursor {
                style::input_cursor_style()
            } else if selection.is_some_and(|(start, end)| index >= start && index < end) {
                style::selected_text_style(text_color)
            } else {
                Style::default().fg(text_color)
            };

            current_spans.push(Span::styled(ch.to_string(), item_style));
            current_line_len += ch_width;
        }

        let input_len = self.text.chars().count();
        let cursor_at_end = self.cursor_visible_status && self.cursor == input_len;

        if cursor_at_end {
            current_spans.push(Span::styled(" ", style::input_cursor_style()));
        }

        content_lines.push(Line::from(current_spans));

        content_lines.into_iter().skip(vertical_scroll as usize).take(area.height as usize).collect()
    }

    pub fn update_cursot_status(&mut self, has_popup: bool) {
        if has_popup {
            self.cursor_visible_status = false;
            return;
        }
        if !self.cursor_visible {
            self.cursor_visible_status = false;
            return;
        }
        if self.cursor_blink_tick.tick() {
            self.cursor_visible_status = !self.cursor_visible_status
        }
    }

    // Text metrics

    pub fn text_char_count(&self) -> usize {
        self.text.chars().count()
    }

    pub fn text_height(&self, input_width: u16) -> u16 {
        let input_chars = self.text_char_count();
        let (_, column) = self.text_position_at(input_chars, input_width);

        let line = if column == 0 && input_chars > 0 {
            self.text_position_at(input_chars.saturating_sub(1), input_width).0
        } else {
            self.text_position_at(input_chars, input_width).0
        };

        (line + 1).max(1) as u16
    }

    pub fn text_position_at(&self, index: usize, input_width: u16) -> (usize, usize) {
        let line_width = input_width.max(1) as usize;
        let mut line = 0usize;
        let mut column = 0usize;

        for ch in self.text.chars().take(index) {
            let ch_width = ch.width().unwrap_or(0).max(1);

            if column > 0 && column.saturating_add(ch_width) > line_width {
                line += 1;
                column = 0;
            }

            column += ch_width;
        }

        (line, column)
    }

    pub fn contents_byte_index_for_char(&self, char_index: usize) -> usize {
        self.text.char_indices().nth(char_index).map(|(index, _)| index).unwrap_or(self.text.len())
    }

    pub fn vertical_scroll(&self, input_width: u16, input_height: u16) -> u16 {
        if self.cursor_following {
            self.vertical_scroll_for_cursor(input_width, input_height)
        } else {
            self.scroll.min(self.max_scroll(input_width, input_height))
        }
    }

    pub fn vertical_scroll_for_cursor(&self, input_width: u16, input_height: u16) -> u16 {
        let cursor_line = self.text_position_at(self.cursor, input_width).0;
        let input_height = input_height.max(1) as usize;

        cursor_line.saturating_add(1).saturating_sub(input_height) as u16
    }

    pub fn max_scroll(&self, input_width: u16, input_height: u16) -> u16 {
        self.text_height(input_width).saturating_sub(input_height.max(1))
    }

    pub fn previous_space_boundary(&self, cursor_position: usize) -> usize {
        let chars: Vec<char> = self.text.chars().collect();
        let mut index = cursor_position.min(chars.len());

        while index > 0 && chars[index - 1] == ' ' {
            index -= 1;
        }

        while index > 0 && chars[index - 1] != ' ' {
            index -= 1;
        }

        index
    }

    pub fn next_space_boundary(&self, cursor_position: usize) -> usize {
        let chars: Vec<char> = self.text.chars().collect();
        let mut index = cursor_position.min(chars.len());

        while index < chars.len() && chars[index] != ' ' {
            index += 1;
        }

        while index < chars.len() && chars[index] == ' ' {
            index += 1;
        }

        index
    }

    pub fn input_index_at_display_position(&self, target_row: usize, target_column: usize, input_width: u16) -> usize {
        let line_width = input_width.max(1) as usize;
        let mut row = 0usize;
        let mut column = 0usize;

        for (index, ch) in self.text.chars().enumerate() {
            let ch_width = ch.width().unwrap_or(0).max(1);

            if column > 0 && column.saturating_add(ch_width) > line_width {
                row += 1;
                column = 0;
            }

            if row > target_row || (row == target_row && target_column <= column.saturating_add(ch_width / 2)) {
                return index;
            }

            column += ch_width;

            if row == target_row && target_column < column {
                return index + 1;
            }
        }

        self.text.chars().count()
    }

    // Editing
    pub fn take_text(&mut self) -> String {
        let text = std::mem::take(&mut self.text);
        self.cursor = 0;
        self.reset_cursor_blink();
        text
    }

    pub fn select_all(&mut self) {
        let end = self.text_char_count();
        self.selection = Some(TextSelection { anchor: 0, focus: end });
        self.cursor = end;
        self.reset_cursor_blink();
    }

    pub fn reset_cursor_blink(&mut self) {
        self.cursor_visible = true;
        self.cursor_following = true;
        self.cursor_visible_status = true;
        self.cursor_blink_tick.reset();
    }

    pub fn copy_selection(&mut self) {
        let Some(selection) = self.selection.as_ref() else {
            return;
        };
        let Some(text) = selection.copy(&self.text) else {
            return;
        };
        let _ = Clipboard::new().and_then(|mut clipboard| clipboard.set_text(text));
    }

    pub fn cut_selection(&mut self) {
        let Some(selection) = self.selection.as_ref() else {
            return;
        };
        let (start, end) = selection.range();
        if start == end {
            self.clear_selection();
            return;
        }
        let Some(text) = selection.copy(&self.text) else {
            return;
        };
        if Clipboard::new().and_then(|mut clipboard| clipboard.set_text(text)).is_ok() {
            self.replace_selection("");
            self.reset_cursor_blink();
        }
    }

    pub fn clear_selection(&mut self) {
        self.selection = None;
    }

    pub fn replace_selection(&mut self, replacement: &str) {
        let Some(selection) = self.selection.take() else {
            self.insert_text(replacement);
            self.clear_selection();
            return;
        };

        let (start, end) = selection.range();

        if start == end {
            self.cursor = start;
            self.insert_text(replacement);
            self.clear_selection();
            return;
        }

        let start_byte = self.contents_byte_index_for_char(start);
        let end_byte = self.contents_byte_index_for_char(end);

        self.text.replace_range(start_byte..end_byte, replacement);
        self.cursor = start + replacement.chars().count();
        self.clear_selection();
        self.reset_cursor_blink();
    }

    pub fn insert_text(&mut self, text: &str) {
        let byte_index = self.contents_byte_index_for_char(self.cursor);
        self.text.insert_str(byte_index, text);
        self.cursor += text.chars().count();
        self.reset_cursor_blink();
    }

    pub fn paste(&mut self) {
        let Ok(mut clipboard) = Clipboard::new() else {
            return;
        };

        let Ok(text) = clipboard.get_text() else {
            return;
        };

        self.replace_selection(&text);
    }

    pub fn input_char(&mut self, ch: char) {
        if self.selection.is_some() {
            self.replace_selection("");
        }

        let byte_index = self.contents_byte_index_for_char(self.cursor);
        self.text.insert(byte_index, ch);
        self.cursor += 1;
        self.reset_cursor_blink();
    }

    pub fn move_left(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
        self.clear_selection();
        self.reset_cursor_blink();
    }

    pub fn move_right(&mut self) {
        self.cursor = self.cursor.saturating_add(1).min(self.text_char_count());
        self.clear_selection();
        self.reset_cursor_blink();
    }

    pub fn move_up(&mut self, input_width: u16) {
        let (line, column) = self.text_position_at(self.cursor, input_width);

        self.cursor = if line == 0 { 0 } else { self.input_index_at_display_position(line - 1, column, input_width) };

        self.clear_selection();
        self.reset_cursor_blink();
    }

    pub fn move_down(&mut self, input_width: u16) {
        let (line, column) = self.text_position_at(self.cursor, input_width);
        self.cursor = self.input_index_at_display_position(line + 1, column, input_width);
        self.clear_selection();
        self.reset_cursor_blink();
    }

    pub fn move_left_ctrl(&mut self) {
        self.cursor = self.previous_space_boundary(self.cursor);
        self.clear_selection();
        self.reset_cursor_blink();
        self.scroll = 0;
    }

    pub fn move_right_ctrl(&mut self) {
        self.cursor = self.next_space_boundary(self.cursor);
        self.clear_selection();
        self.reset_cursor_blink();
        self.scroll = self.vertical_scroll_for_cursor(1, 1);
    }

    pub fn move_up_ctrl(&mut self) {
        self.cursor = 0;
        self.clear_selection();
        self.reset_cursor_blink();
    }

    pub fn move_down_ctrl(&mut self) {
        self.cursor = self.text_char_count();
        self.clear_selection();
        self.reset_cursor_blink();
    }

    pub fn backspace(&mut self) {
        if self.selection.is_some() {
            self.replace_selection("");
            self.reset_cursor_blink();
            return;
        }

        if self.cursor == 0 {
            return;
        }

        let remove_at = self.contents_byte_index_for_char(self.cursor - 1);
        self.text.remove(remove_at);
        self.cursor -= 1;
        self.reset_cursor_blink();
    }

    pub fn delete(&mut self) {
        if self.selection.is_some() {
            self.replace_selection("");
            self.reset_cursor_blink();
            return;
        }

        if self.cursor >= self.text_char_count() {
            return;
        }

        let remove_at = self.contents_byte_index_for_char(self.cursor);
        self.text.remove(remove_at);
        self.reset_cursor_blink();
    }

    pub fn backspace_ctrl(&mut self) {
        if self.selection.is_some() {
            self.replace_selection("");
            self.reset_cursor_blink();
            return;
        }

        let start = self.previous_space_boundary(self.cursor);

        if start == self.cursor {
            return;
        }

        let start_byte = self.contents_byte_index_for_char(start);
        let end_byte = self.contents_byte_index_for_char(self.cursor);

        self.text.replace_range(start_byte..end_byte, "");
        self.cursor = start;
        self.reset_cursor_blink();
    }

    pub fn delete_ctrl(&mut self) {
        if self.selection.is_some() {
            self.replace_selection("");
            self.reset_cursor_blink();
            return;
        }

        let end = self.next_space_boundary(self.cursor);

        if end == self.cursor {
            return;
        }

        let start_byte = self.contents_byte_index_for_char(self.cursor);
        let end_byte = self.contents_byte_index_for_char(end);

        self.text.replace_range(start_byte..end_byte, "");
        self.reset_cursor_blink();
    }

    // Mouse selection

    pub fn begin_selection(&mut self, index: usize) {
        let index = index.min(self.text_char_count());
        self.selection = Some(TextSelection::new(index));
        self.cursor = index;
        self.reset_cursor_blink();
    }

    pub fn update_selection(&mut self, index: usize) {
        let index = index.min(self.text_char_count());

        if let Some(selection) = &mut self.selection {
            selection.focus = index;
        } else {
            self.selection = Some(TextSelection::new(index));
        }

        self.cursor = index;
        self.reset_cursor_blink();
    }

    pub fn char_index(&self, mouse_column: u16, mouse_row: u16, area: Rect) -> Option<usize> {
        if mouse_row < area.y || mouse_row >= area.y.saturating_add(area.height) || mouse_column < area.x || mouse_column >= area.x.saturating_add(area.width) {
            return None;
        }
        let vertical_scroll = self.vertical_scroll(area.width, area.height) as usize;
        let target_row = mouse_row.saturating_sub(area.y) as usize + vertical_scroll;
        let target_column = mouse_column.saturating_sub(area.x) as usize;
        Some(self.input_index_at_display_position(target_row, target_column, area.width))
    }

    pub fn scroll_to(&mut self, lines: i32, input_width: u16, input_height: u16) {
        let max_scroll = self.max_scroll(input_width, input_height);

        self.scroll = if lines < 0 {
            self.scroll.saturating_sub(lines.unsigned_abs() as u16)
        } else {
            self.scroll.saturating_add(lines as u16).min(max_scroll)
        };

        self.cursor_following = false;
    }

    // Events

    pub fn key_event(&mut self, key: KeyEvent, input_width: u16) -> bool {
        match key.code {
            KeyCode::Char('a') if key.modifiers.contains(KeyModifiers::CONTROL) => self.select_all(),
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => self.copy_selection(),
            KeyCode::Char('x') if key.modifiers.contains(KeyModifiers::CONTROL) => self.cut_selection(),
            KeyCode::Char('v') if key.modifiers.contains(KeyModifiers::CONTROL) => self.paste(),

            KeyCode::Char('w') if key.modifiers.contains(KeyModifiers::CONTROL) => self.backspace_ctrl(),
            KeyCode::Backspace if key.modifiers.contains(KeyModifiers::CONTROL) => self.backspace_ctrl(),
            KeyCode::Delete if key.modifiers.contains(KeyModifiers::CONTROL) => self.delete_ctrl(),
            KeyCode::Backspace => self.backspace(),
            KeyCode::Delete => self.delete(),

            KeyCode::Left if key.modifiers.contains(KeyModifiers::CONTROL) => self.move_left_ctrl(),
            KeyCode::Left => self.move_left(),
            KeyCode::Right if key.modifiers.contains(KeyModifiers::CONTROL) => self.move_right_ctrl(),
            KeyCode::Right => self.move_right(),

            KeyCode::Up if key.modifiers.contains(KeyModifiers::CONTROL) => self.move_up_ctrl(),
            KeyCode::Up => self.move_up(input_width),
            KeyCode::Down if key.modifiers.contains(KeyModifiers::CONTROL) => self.move_down_ctrl(),
            KeyCode::Down => self.move_down(input_width),

            KeyCode::Char(ch) => self.input_char(ch),
            _ => return false,
        }

        true
    }

    pub fn mouse_event(&mut self, mouse: MouseEvent, area: Rect) -> bool {
        match mouse.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                if let Some(index) = self.char_index(mouse.column, mouse.row, area) {
                    self.begin_selection(index);
                    return true;
                }
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                if let Some(index) = self.char_index(mouse.column, mouse.row, area) {
                    self.update_selection(index);
                    return true;
                }
            }
            MouseEventKind::ScrollUp => {
                self.scroll_to(-Self::MOUSE_SCROLL_LINES, area.width, area.height);
                return true;
            }
            MouseEventKind::ScrollDown => {
                self.scroll_to(Self::MOUSE_SCROLL_LINES, area.width, area.height);
                return true;
            }
            _ => {}
        }

        false
    }
}
