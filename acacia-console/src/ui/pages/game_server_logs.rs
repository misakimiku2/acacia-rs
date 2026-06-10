use crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};
use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::ui::{
    components::style::{self, ACCENT_BAR},
    pages::{Page, PopupPage},
};
use crate::{
    base::utils::strip_ansi,
    base::game_server::game_server_base::{GameServerEvent, GameServerMessage},
    ui::pages::game_server_log::GameServerLogPopupPage,
};

pub struct GameServerLogsPage {
    pub logs: Vec<GameServerMessage>,
    scroll: u16,
    follow_tail: bool,
}

impl GameServerLogsPage {
    const MOUSE_SCROLL_LINES: i32 = 3;
    const HORIZONTAL_EXPANSION: u16 = 2;

    pub fn new() -> Self {
        Self {
            logs: Vec::new(),
            scroll: 0,
            follow_tail: true,
        }
    }

    pub fn reader(&self, frame: &mut Frame, area: Rect, has_popup: bool) {
        let log_area = Rect {
            x: area.x.saturating_sub(Self::HORIZONTAL_EXPANSION),
            y: area.y,
            width: area.width.saturating_add(Self::HORIZONTAL_EXPANSION.saturating_mul(2)),
            height: area.height,
        };
        let log_scroll = self.visible_scroll(log_area.height) as usize;
        let lines = self
            .logs
            .iter()
            .skip(log_scroll)
            .take(log_area.height as usize)
            .map(|(event, message)| Self::game_server_log_line(*event, message, Some(log_area.width), has_popup))
            .collect::<Vec<_>>();
        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, log_area);
    }

    pub fn event(&mut self, event: Event, area: Rect, page: &mut Page, popup_page: &mut PopupPage, game_server_log_popup_page: &mut GameServerLogPopupPage) -> bool {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Esc => *page = Page::Console,
                KeyCode::Up => self.scroll_logs(-1, area.height),
                KeyCode::Down => self.scroll_logs(1, area.height),
                KeyCode::PageUp => self.scroll_logs(-(area.height as i32), area.height),
                KeyCode::PageDown => self.scroll_logs(area.height as i32, area.height),
                _ => {}
            },
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::ScrollUp => self.scroll_logs(-Self::MOUSE_SCROLL_LINES, area.height),
                MouseEventKind::ScrollDown => self.scroll_logs(Self::MOUSE_SCROLL_LINES, area.height),
                MouseEventKind::Down(MouseButton::Left) => {
                    if let Some(index) = self.clicked_log_index(mouse.row, area.y, area.height) {
                        game_server_log_popup_page.load_log(index, &self.logs);
                        *popup_page = PopupPage::GameServerLog;
                    }
                }
                _ => {}
            },
            _ => {}
        }
        false
    }

    fn clicked_log_index(&self, mouse_row: u16, area_y: u16, visible_height: u16) -> Option<usize> {
        let relative_row = mouse_row.checked_sub(area_y)?;
        if relative_row >= visible_height {
            return None;
        }

        let index = self.visible_scroll(visible_height) as usize + relative_row as usize;
        (index < self.logs.len()).then_some(index)
    }

    fn visible_scroll(&self, visible_height: u16) -> u16 {
        let max_scroll = self.max_scroll(visible_height);
        if self.follow_tail { max_scroll } else { self.scroll.min(max_scroll) }
    }

    fn scroll_logs(&mut self, lines: i32, visible_height: u16) {
        let max_scroll = self.max_scroll(visible_height);
        self.scroll = if self.follow_tail { max_scroll } else { self.scroll.min(max_scroll) };
        if lines < 0 {
            self.scroll = self.scroll.saturating_sub(lines.unsigned_abs() as u16);
        } else {
            self.scroll = self.scroll.saturating_add(lines as u16).min(max_scroll);
        }
        self.follow_tail = self.scroll >= max_scroll;
    }

    fn max_scroll(&self, visible_height: u16) -> u16 {
        self.logs.len().saturating_sub(visible_height as usize) as u16
    }

    fn game_server_log_line(event: GameServerEvent, message: &str, content_width: Option<u16>, has_popup: bool) -> Line<'static> {
        let body = strip_ansi(message);
        let text_color = style::normal_text_color(has_popup);
        let muted_color = style::muted_text_color(has_popup);
        let accent = match event {
            GameServerEvent::Debug => style::debug_accent_color(has_popup),
            GameServerEvent::Info => style::info_accent_color(has_popup),
            GameServerEvent::Warn => style::warn_accent_color(has_popup),
            GameServerEvent::Error | GameServerEvent::StdError | GameServerEvent::InError => style::error_accent_color(has_popup),
            GameServerEvent::Line => muted_color,
        };
        let background = match event {
            GameServerEvent::StdError | GameServerEvent::InError => Some(style::error_background_color(has_popup)),
            _ => None,
        };
        let (visible, truncated, padding) = if let Some(content_width) = content_width {
            let accent_width = 2;
            let body_width = content_width.saturating_sub(accent_width);
            let (visible, truncated) = Self::truncate_log_preview(body, body_width);
            let padding = if background.is_some() {
                let body_len = visible.chars().count() as u16 + if truncated { 3 } else { 0 };
                body_width.saturating_sub(body_len)
            } else {
                0
            };
            (visible, truncated, padding)
        } else {
            (body, false, 0)
        };

        let body_style = background.map_or_else(|| Style::default().fg(text_color), |background| Style::default().fg(text_color).bg(background));
        let ellipsis_style = background.map_or_else(|| Style::default().fg(muted_color), |background| Style::default().fg(muted_color).bg(background));
        let mut spans = vec![Span::styled(ACCENT_BAR, Style::default().fg(accent)), Span::styled(" ", body_style)];
        spans.push(Span::styled(visible, body_style));
        if truncated {
            spans.push(Span::styled("...", ellipsis_style));
        }
        if padding > 0 {
            spans.push(Span::styled(" ".repeat(padding as usize), body_style));
        }

        Line::from(spans)
    }

    fn truncate_log_preview(log: String, content_width: u16) -> (String, bool) {
        let max_chars = content_width.max(1) as usize;
        let char_count = log.chars().count();
        if char_count <= max_chars {
            return (log, false);
        }

        let keep = max_chars.saturating_sub(3).max(1);
        (log.chars().take(keep).collect(), true)
    }
}
