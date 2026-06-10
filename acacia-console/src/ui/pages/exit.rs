use crossterm::event::{Event, KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{List, ListItem},
};

use crate::ui::{
    components::{
        popup::{self, PopupLayout},
        style,
    },
    pages::PopupPage,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ExitConfirmChoice {
    Ok,
    Cancel,
}

pub struct ExitPopupPage {
    pub choice: ExitConfirmChoice,
    pub exit_flag: bool,
}

impl ExitPopupPage {
    pub fn new() -> Self {
        Self {
            choice: ExitConfirmChoice::Ok,
            exit_flag: false,
        }
    }

    pub fn window_area(&self, area: Rect) -> Rect {
        PopupLayout::window_area_by_content_size(area, 40, 2)
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let window_area = self.window_area(area);
        let content_area = PopupLayout::content_area(window_area);
        PopupLayout::render(frame, window_area, "Exit");
        self.render_choices(frame, content_area);
    }

    fn render_choices(&self, frame: &mut Frame, content_area: Rect) {
        let selected_background_color = style::selected_item_background(false);
        let selected_text_color = style::selected_item_text_color(false);
        let items = [(ExitConfirmChoice::Ok, "OK"), (ExitConfirmChoice::Cancel, "Cancel")]
            .into_iter()
            .map(|(choice, label)| {
                let text_style = if choice == self.choice {
                    Style::default().fg(selected_text_color).bg(selected_background_color).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(style::normal_text_color(false))
                };
                ListItem::new(format!("   {label}")).style(text_style)
            })
            .collect::<Vec<_>>();
        frame.render_widget(List::new(items), content_area);
    }

    fn choice_at(&self, mouse_column: u16, mouse_row: u16, area: Rect) -> Option<ExitConfirmChoice> {
        let window_area = self.window_area(area);
        let content_area = PopupLayout::content_area(window_area);
        if !popup::rect_contains(content_area, mouse_column, mouse_row) {
            return None;
        }
        match mouse_row.saturating_sub(content_area.y) {
            0 => Some(ExitConfirmChoice::Ok),
            1 => Some(ExitConfirmChoice::Cancel),
            _ => None,
        }
    }

    pub fn event(&mut self, event: Event, area: Rect, current_popup_page: &mut PopupPage) {
        match event {
            Event::Key(key) => self.key_event(key, current_popup_page),
            Event::Mouse(mouse) => self.mouse_event(mouse, area, current_popup_page),
            _ => (),
        }
    }

    fn key_event(&mut self, key: KeyEvent, popup_page: &mut PopupPage) -> () {
        if PopupLayout::key_event(key, popup_page) {
            return;
        };
        match key.code {
            KeyCode::Enter => {
                if self.choice == ExitConfirmChoice::Cancel {
                    *popup_page = PopupPage::None;
                } else {
                    self.exit_flag = true;
                }
            }
            KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                self.choice = match self.choice {
                    ExitConfirmChoice::Ok => ExitConfirmChoice::Cancel,
                    ExitConfirmChoice::Cancel => ExitConfirmChoice::Ok,
                };
            }
            _ => (),
        }
    }

    fn mouse_event(&mut self, mouse: MouseEvent, area: Rect, popup_page: &mut PopupPage) -> () {
        let window_area = self.window_area(area);
        if PopupLayout::mouse_event(mouse, window_area, popup_page) {
            return;
        };
        if let Some(choice) = self.choice_at(mouse.column, mouse.row, area) {
            self.choice = choice;
            if mouse.kind == MouseEventKind::Up(MouseButton::Left) {
                if choice == ExitConfirmChoice::Cancel {
                    *popup_page = PopupPage::None;
                } else {
                    self.exit_flag = true;
                }
            }
        }
    }
}
