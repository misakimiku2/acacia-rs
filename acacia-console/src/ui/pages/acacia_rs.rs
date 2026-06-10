use crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::ui::pages::PopupPage;

pub struct AcaciaRsPopupPage;

impl AcaciaRsPopupPage {
    const TITLE: [&str; 2] = ["▄▀█ █▀▀ ▄▀█ █▀▀ █ ▄▀█ ▄▄ █▀█ █▀▀", "█▀█ █▄▄ █▀█ █▄▄ █ █▀█    █▀▄ ▄▄█"];

    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let content_width = Self::TITLE.iter().map(|line| line.chars().count()).max().unwrap_or_default() as u16;
        let content_height = Self::TITLE.len() as u16;
        let content = Rect {
            x: area.x + area.width.saturating_sub(content_width) / 2,
            y: area.y + area.height.saturating_sub(content_height) / 2,
            width: content_width.min(area.width),
            height: content_height.min(area.height),
        };
        let lines = Self::TITLE
            .into_iter()
            .map(|line| Line::from(Span::styled(line, Style::default().fg(Color::Rgb(238, 238, 238)).add_modifier(Modifier::BOLD))))
            .collect::<Vec<_>>();
        let overlay = Paragraph::new(lines).alignment(Alignment::Center);
        frame.render_widget(overlay, content);
    }

    pub fn event(&mut self, event: Event, _area: Rect, popup_page: &mut PopupPage) -> bool {
        match event {
            Event::Key(key) if key.code == KeyCode::Esc => *popup_page = PopupPage::None,
            Event::Mouse(mouse) if mouse.kind == MouseEventKind::Down(MouseButton::Left) => *popup_page = PopupPage::None,
            _ => {}
        }
        false
    }
}
