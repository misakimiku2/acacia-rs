use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};

use crate::ui::pages::PageManager;

pub struct ConsolePage {
    input: String,
    history_list: Vec<String>,
    history_selected: Option<usize>,
}

impl ConsolePage {
    const MENU_MAX_HEIGHT: u16 = 10;

    pub fn new() -> Self {
        Self {
            input: String::new(),
            history_list: Vec::new(),
            history_selected: None,
        }
    }

    pub fn reader(&self, frame: &mut Frame, area: Rect) {
        let bottom_popup_height = self.menu_height();
        let chunks = Layout::default().direction(Direction::Vertical).constraints([
            Constraint::Min(5),
            Constraint::Length(1), // bottom margin
            Constraint::Length(self.menu_height()),
            Constraint::Length(self.menu_height()),
            Constraint::Length(2), // footer height
        ]);
    }

    pub fn input_height(&self, area_height: u16, input_width: u16) -> u16 {
        let max_height = area_height.saturating_sub(5).clamp(1, 10);
        self.input_line_count(input_width).saturating_add(3).min(max_height).max(1)
    }
    pub fn input_line_count(&self, input_width: u16) -> u16 {
        let input_chars = self.input_char_count();
        let (_, column) = input::input_line_and_column(&self.input, input_chars, input_width);
        let line = if column == 0 && input_chars > 0 {
            input::input_line_and_column(&self.input, input_chars.saturating_sub(1), input_width).0
        } else {
            input::input_line_and_column(&self.input, input_chars, input_width).0
        };
        (line + 1).max(1) as u16
    }

    pub fn menu_height(&self) -> u16 {
        if !self.show_menu() {
            return 0;
        }

        if self.show_history_menu() {
            return self.history_list.len().max(1).min(Self::MENU_MAX_HEIGHT as usize) as u16;
        }

        if self.show_command_menu() {
            return 0; // command_popup_item_count(&self.input).max(1).min(Self::MENU_MAX_HEIGHT as usize) as u16;
        }

        0
    }

    pub fn show_menu(&self) -> bool {
        self.show_history_menu() || self.show_command_menu()
    }

    pub fn show_command_menu(&self) -> bool {
        !self.show_history_menu() && self.input.starts_with('/')
    }

    pub fn show_history_menu(&self) -> bool {
        self.history_selected.is_some()
    }
}
