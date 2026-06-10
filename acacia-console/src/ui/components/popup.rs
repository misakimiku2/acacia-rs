use crossterm::event::{KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Clear, Paragraph},
};

use crate::ui::{
    components::style::{OVERLAY_POPUP_BACKGROUND, overlay_hint_color},
    pages::PopupPage,
};

pub struct PopupLayout {}

impl PopupLayout {
    pub fn window_area_by_content_size(area: Rect, content_width: u16, content_height: u16) -> Rect {
        let window_width = content_width.saturating_add(2).min(area.width.saturating_sub(4));
        let window_height = content_height.saturating_add(5).min(area.height.saturating_sub(2));
        Rect {
            x: area.x + area.width.saturating_sub(window_width) / 2,
            y: area.y + area.height.saturating_sub(window_height) / 2,
            width: window_width,
            height: window_height,
        }
    }

    pub fn content_area(window_area: Rect) -> Rect {
        Rect {
            x: window_area.x + 1,
            y: window_area.y + 3,
            width: window_area.width.saturating_sub(2),
            height: window_area.height.saturating_sub(5),
        }
    }

    pub fn render(frame: &mut Frame, window_area: Rect, title: &str) {
        frame.render_widget(Clear, window_area);
        frame.render_widget(Block::default().style(Style::default().bg(OVERLAY_POPUP_BACKGROUND)), window_area);

        if window_area.height > 1 {
            let title_area = Rect {
                x: window_area.x + 4,
                y: window_area.y + 1,
                width: window_area.width.saturating_sub(4),
                height: 1,
            };
            let title_message = Paragraph::new(Line::styled(title, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)));
            frame.render_widget(title_message, title_area);

            let escape_area = Rect {
                x: window_area.x + window_area.width.saturating_sub(3 + 4),
                y: window_area.y + 1,
                width: 3,
                height: 1,
            };
            let escape_message = Paragraph::new(Line::styled("esc", Style::default().fg(overlay_hint_color())));
            frame.render_widget(escape_message, escape_area);
        }
    }

    pub fn key_event(key: KeyEvent, popup_page: &mut PopupPage) -> bool {
        match key.code {
            KeyCode::Esc => {
                *popup_page = PopupPage::None;
                true
            }
            _ => false,
        }
    }

    pub fn mouse_event(mouse: MouseEvent, window_area: Rect, popup_page: &mut PopupPage) -> bool {
        if mouse.kind == MouseEventKind::Down(MouseButton::Left) && !rect_contains(window_area, mouse.column, mouse.row) {
            *popup_page = PopupPage::None;
            return true;
        }
        let escape_area = Rect {
            x: window_area.x + window_area.width.saturating_sub(3 + 4),
            y: window_area.y + 1,
            width: 3,
            height: 1,
        };
        if rect_contains(escape_area, mouse.column, mouse.row) {
            if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
                *popup_page = PopupPage::None;
            }
            return true;
        }
        false
    }
}

pub fn rect_contains(area: Rect, x: u16, y: u16) -> bool {
    x >= area.x && x < area.x.saturating_add(area.width) && y >= area.y && y < area.y.saturating_add(area.height)
}
