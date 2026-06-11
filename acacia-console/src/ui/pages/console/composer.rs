use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::ui::components::{input::InputLayout, style};

pub struct ConsoleComposerLayout {
    pub input: InputLayout,
}

impl ConsoleComposerLayout {
    const BORDER_MARGIN: u16 = 2;
    const TEXT_PADDING: u16 = 2;

    pub fn new() -> Self {
        Self { input: InputLayout::new() }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, has_popup: bool) {
        let input_area = self.input_area(area);
        frame.render_widget(Paragraph::new(self.render_background(area.width, input_area.height, has_popup)), area);
        frame.render_widget(Paragraph::new(self.input.render(input_area, has_popup)), input_area);
    }

    pub fn render_background(&self, area_width: u16, input_height: u16, has_popup: bool) -> Vec<Line<'static>> {
        let accent_color = style::normal_accent_color(has_popup);
        let text_color = style::normal_text_color(has_popup);
        let background_color = style::normal_background_active_color(has_popup);
        let background_style = Style::default().fg(text_color).bg(background_color);

        let mut lines = Vec::new();
        lines.push(Self::render_background_line(area_width, accent_color, background_style));
        for _ in 0..input_height {
            lines.push(Self::render_background_line(area_width, accent_color, background_style));
        }
        lines.push(Self::render_background_line(area_width, accent_color, background_style));
        lines.push(Self::render_background_bottom_line(area_width, accent_color, background_color));
        lines
    }

    pub fn render_background_line(area_width: u16, accent_color: Color, background_style: Style) -> Line<'static> {
        let left_margin = " ".repeat(Self::BORDER_MARGIN as usize);
        let rest_width = area_width.saturating_sub(Self::BORDER_MARGIN * 2 + 1) as usize;
        let right_margin = " ".repeat(Self::BORDER_MARGIN as usize);
        Line::from(vec![
            Span::raw(left_margin),
            Span::styled("┃", Style::default().fg(accent_color)),
            Span::styled(" ".repeat(rest_width), background_style),
            Span::raw(right_margin),
        ])
    }

    pub fn render_background_bottom_line(area_width: u16, accent_color: Color, background_color: Color) -> Line<'static> {
        let left_margin = " ".repeat(Self::BORDER_MARGIN as usize);
        let border = "▀".repeat(area_width.saturating_sub(Self::BORDER_MARGIN * 2 + 1) as usize);
        let right_margin = " ".repeat(Self::BORDER_MARGIN as usize);
        Line::from(vec![
            Span::raw(left_margin),
            Span::styled("╹", Style::default().fg(accent_color)),
            Span::styled(border, Style::default().fg(background_color)),
            Span::raw(right_margin),
        ])
    }

    pub fn height(&self, area: Rect) -> u16 {
        let max_height = area.height.saturating_sub(7).clamp(1, 10);
        let input_width = self.input_width(area.width);
        self.input.text_height(input_width).saturating_add(3).clamp(1, max_height)
    }

    pub fn input_area(&self, area: Rect) -> Rect {
        Rect {
            x: area.x.saturating_add(Self::BORDER_MARGIN + 1 + Self::TEXT_PADDING),
            y: area.y.saturating_add(1),
            width: self.input_width(area.width),
            height: self.input_height(area.height),
        }
    }

    pub fn input_width(&self, area_width: u16) -> u16 {
        area_width.saturating_sub(Self::BORDER_MARGIN * 2 + Self::TEXT_PADDING * 2 + 1).max(1)
    }

    pub fn input_height(&self, area_height: u16) -> u16 {
        area_height.saturating_sub(3).max(1)
    }

    // Events

    pub fn key_event(&mut self, key: KeyEvent, area: Rect) -> bool {
        let input_width = self.input_width(area.width);
        self.input.key_event(key, input_width)
    }

    pub fn mouse_event(&mut self, mouse: MouseEvent, area: Rect) {
        let input_area = self.input_area(area);
        self.input.mouse_event(mouse, input_area);
    }
}
