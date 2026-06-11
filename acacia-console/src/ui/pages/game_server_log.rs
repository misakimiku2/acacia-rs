use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::{
    base::{
        game_server::game_server_base::{GameServerEvent, GameServerMessage},
        utils::strip_ansi,
    },
    ui::{
        components::{input::InputLayout, popup::PopupLayout, style},
        pages::PopupPage,
    },
};

pub struct GameServerLogPopupPage {
    pub input: InputLayout,
    pub event: GameServerEvent,
    pub log_index: usize,
}

impl GameServerLogPopupPage {
    const INPUT_HORIZONTAL_PADDING: u16 = 3;
    const FOOTER_HEIGHT: u16 = 2;

    pub fn new() -> Self {
        Self {
            input: InputLayout::new(),
            event: GameServerEvent::Line,
            log_index: 0,
        }
    }

    pub fn load_log(&mut self, index: usize, logs: &[GameServerMessage]) {
        let Some((event, message)) = logs.get(index) else {
            return;
        };

        self.log_index = index;
        self.event = *event;
        self.input.clear_selection();
        self.input.text = strip_ansi(message);
        self.input.cursor = self.input.text_char_count();
        self.input.reset_cursor_blink();
    }

    pub fn window_area(&self, area: Rect) -> Rect {
        let window_width = area.width.saturating_sub(4);
        let window_height = area.height.saturating_sub(2);
        Rect {
            x: area.x + area.width.saturating_sub(window_width) / 2,
            y: area.y + area.height.saturating_sub(window_height) / 2,
            width: window_width,
            height: window_height,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let window_area = self.window_area(area);
        let content_area = PopupLayout::content_area(window_area);
        let input_area = Self::input_area(content_area);
        let footer_area = Self::footer_area(content_area);
        PopupLayout::render(frame, window_area, "Log");
        frame.render_widget(Block::default().style(Style::default().bg(style::global_background(false))), input_area);
        frame.render_widget(Paragraph::new(self.input.render(input_area, false)), input_area);
        self.render_footer(frame, footer_area);
    }

    fn input_area(content_area: Rect) -> Rect {
        Rect {
            x: content_area.x.saturating_add(Self::INPUT_HORIZONTAL_PADDING),
            y: content_area.y,
            width: content_area.width.saturating_sub(Self::INPUT_HORIZONTAL_PADDING.saturating_mul(2)),
            height: content_area.height.saturating_sub(Self::FOOTER_HEIGHT),
        }
    }

    fn footer_area(content_area: Rect) -> Rect {
        let input_area = Self::input_area(content_area);
        Rect {
            x: input_area.x,
            y: input_area.y.saturating_add(input_area.height),
            width: input_area.width,
            height: Self::FOOTER_HEIGHT.min(content_area.height),
        }
    }

    fn render_footer(&self, frame: &mut Frame, footer_area: Rect) {
        if footer_area.width == 0 || footer_area.height == 0 {
            return;
        }

        let level = self.event_label();
        let action = "up / down";
        let gap = footer_area.width.saturating_sub(level.len() as u16 + action.len() as u16) as usize;
        let line = Line::from(vec![
            Span::styled(level, Style::default().fg(self.event_color()).add_modifier(Modifier::BOLD)),
            Span::raw(" ".repeat(gap)),
            Span::styled(action, Style::default().fg(style::muted_text_color(false))),
        ]);

        let line_area = Rect {
            x: footer_area.x,
            y: footer_area.y.saturating_add(1),
            width: footer_area.width,
            height: 1,
        };
        frame.render_widget(Paragraph::new(line), line_area);
    }

    fn event_label(&self) -> &'static str {
        match self.event {
            GameServerEvent::Debug => "DEBUG",
            GameServerEvent::Info => "INFO",
            GameServerEvent::Warn => "WARN",
            GameServerEvent::Error => "ERROR",
            GameServerEvent::StdError => "RUST ERROR",
            GameServerEvent::InError => "RUST ERROR",
            GameServerEvent::Line => "NORMAL",
        }
    }

    fn event_color(&self) -> Color {
        match self.event {
            GameServerEvent::Debug => style::debug_accent_color(false),
            GameServerEvent::Info => style::info_accent_color(false),
            GameServerEvent::Warn => style::warn_accent_color(false),
            GameServerEvent::Error | GameServerEvent::StdError | GameServerEvent::InError => style::error_accent_color(false),
            GameServerEvent::Line => style::muted_text_color(false),
        }
    }

    pub fn event(&mut self, event: Event, area: Rect, current_popup_page: &mut PopupPage, logs: &[GameServerMessage]) {
        match event {
            Event::Key(key) => self.key_event(key, area, current_popup_page, logs),
            Event::Mouse(mouse) => self.mouse_event(mouse, area, current_popup_page),
            _ => (),
        }
    }

    fn key_event(&mut self, key: KeyEvent, area: Rect, popup_page: &mut PopupPage, logs: &[GameServerMessage]) -> () {
        let window_area = self.window_area(area);
        if PopupLayout::key_event(key, popup_page) {
            return;
        };
        match key.code {
            KeyCode::Up => {
                self.load_log(self.log_index.saturating_sub(1), logs);
                return;
            }
            KeyCode::Down => {
                self.load_log(self.log_index.saturating_add(1).min(logs.len().saturating_sub(1)), logs);
                return;
            }
            _ => (),
        }
        let content_area = PopupLayout::content_area(window_area);
        let input_area = Self::input_area(content_area);
        if self.input.key_event(key, input_area.width) {
            return;
        }
        match key.code {
            _ => (),
        }
    }

    fn mouse_event(&mut self, mouse: MouseEvent, area: Rect, popup_page: &mut PopupPage) -> () {
        let window_area = self.window_area(area);
        if PopupLayout::mouse_event(mouse, window_area, popup_page) {
            return;
        };
        let content_area = PopupLayout::content_area(window_area);
        let input_area = Self::input_area(content_area);
        if self.input.mouse_event(mouse, input_area) {
            return;
        }
    }
}
