use arboard::Clipboard;
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::ui::components::{
    popup,
    selection::{TextSelection, char_index_at_display_column},
    style::{self, ACCENT_BAR},
    text::{fit_text_to_width, wrap_text_by_width},
};

pub enum ConsoleMessage {
    Info(String),
    User(String),
    Ret(String),
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ConsoleMessageLineType {
    Info,
    User,
    Ret,
    #[default]
    Spacer,
}

#[derive(Default, Debug)]
pub struct ConsoleMessageLine {
    id: usize,
    r#type: ConsoleMessageLineType,
    text: String,
}

impl ConsoleMessageLine {
    const BORDER_MARGIN: usize = 2;
    const TEXT_PADDING: usize = 2;

    fn text_start_column(&self) -> usize {
        if self.text.is_empty() { 0 } else { Self::BORDER_MARGIN + Self::TEXT_PADDING + 1 }
    }

    fn selectable_len(&self) -> usize {
        self.text.chars().count()
    }

    fn selectable_index_at_column(&self, column: usize) -> usize {
        let selectable_len = self.selectable_len();
        if selectable_len == 0 {
            return 0;
        }
        let content_column = column.saturating_sub(self.text_start_column());
        char_index_at_display_column(&self.text, content_column).min(selectable_len)
    }
}

pub struct ConsoleConversationLayout {
    pub follow_tail: bool,
    pub scroll: u16,
    pub messages: Vec<ConsoleMessage>,
    pub selection: Option<TextSelection>,
    pub hovered_group: Option<usize>,
}

impl ConsoleConversationLayout {
    const MOUSE_SCROLL_LINES: i32 = 3;
    const DEFAULT_INFO: &str = concat!(
        "▄▀█ █▀▀ ▄▀█ █▀▀ █ ▄▀█ ▄▄ █▀█ █▀▀ \n",
        "█▀█ █▄▄ █▀█ █▄▄ █ █▀█    █▀▄ ▄▄█ \n",
        "\n",
        "See full release notes:\n",
        "https://github.com/ahalpha/acacia-rs/releases/latest\n",
        "\n",
        "Tip: type / to show commands",
    );

    pub fn new() -> Self {
        Self {
            follow_tail: true,
            scroll: 0,
            messages: Vec::from([ConsoleMessage::Info(Self::DEFAULT_INFO.to_string())]),
            selection: None,
            hovered_group: None,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect, has_popup: bool) {
        let message_lines_scroll_pos = self.message_lines_scroll_pos(area);
        let rendered_message_lines = self.render_message_lines(area, has_popup);
        frame.render_widget(Paragraph::new(rendered_message_lines).scroll((message_lines_scroll_pos, 0)), area);
    }

    pub fn render_message_lines(&self, area: Rect, has_popup: bool) -> Vec<Line<'static>> {
        let message_lines = self.messages_visual_lines(area.width);
        let selection = self.selection.map(|selection| selection.range()).filter(|(start, end)| start != end);

        let accent = style::normal_accent_color(has_popup);
        let normal = style::normal_text_color(has_popup);
        let user_block_bg = style::normal_background_color(has_popup);
        let active_block_bg = style::normal_background_active_color(has_popup);

        let mut lines = Vec::with_capacity(message_lines.len());
        let mut char_index = 0usize;

        for line in message_lines {
            let background = if line.r#type == ConsoleMessageLineType::User && self.hovered_group == Some(line.id) {
                active_block_bg
            } else {
                user_block_bg
            };

            lines.push(self.messages_visual_line(&line, background, accent, normal, has_popup, area.width, char_index, selection));

            char_index += line.selectable_len() + 1;
        }

        lines
    }

    pub fn message_lines_scroll_pos(&self, area: Rect) -> u16 {
        let message_lines_height = self.message_lines_height(area);
        if self.follow_tail { message_lines_height } else { self.scroll.min(message_lines_height) }
    }

    pub fn message_lines_height(&self, area: Rect) -> u16 {
        self.messages_visual_lines(area.width).len().saturating_sub(area.height as usize) as u16
    }

    pub fn messages_plain_text(&self, area_width: u16) -> String {
        self.messages_visual_lines(area_width).into_iter().map(|line| line.text).collect::<Vec<_>>().join("\n")
    }

    pub fn messages_visual_lines(&self, area_width: u16) -> Vec<ConsoleMessageLine> {
        let content_width = area_width.max(1) as usize;
        let text_width = content_width.saturating_sub(ConsoleMessageLine::BORDER_MARGIN * 2 + ConsoleMessageLine::TEXT_PADDING * 2 + 1).max(1);
        let mut lines = Vec::new();

        for (index, message) in self.messages.iter().enumerate() {
            lines.push(ConsoleMessageLine::default());
            let id = index + 1;
            match message {
                ConsoleMessage::User(input) => {
                    lines.push(ConsoleMessageLine {
                        r#type: ConsoleMessageLineType::User,
                        id,
                        ..Default::default()
                    });
                    for wrapped in wrap_text_by_width(input, text_width) {
                        lines.push(ConsoleMessageLine {
                            text: wrapped,
                            r#type: ConsoleMessageLineType::User,
                            id,
                            ..Default::default()
                        });
                    }
                    lines.push(ConsoleMessageLine {
                        r#type: ConsoleMessageLineType::User,
                        id,
                        ..Default::default()
                    });
                }
                ConsoleMessage::Info(text) => {
                    for wrapped in wrap_text_by_width(text, text_width) {
                        lines.push(ConsoleMessageLine {
                            text: wrapped,
                            r#type: ConsoleMessageLineType::Info,
                            id,
                        });
                    }
                }
                ConsoleMessage::Ret(text) => {
                    for wrapped in wrap_text_by_width(text, text_width) {
                        lines.push(ConsoleMessageLine {
                            text: wrapped,
                            r#type: ConsoleMessageLineType::Ret,
                            id,
                        });
                    }
                }
            }
        }
        lines
    }

    pub fn messages_visual_line(
        &self,
        line: &ConsoleMessageLine,
        background: Color,
        accent: Color,
        text_color: Color,
        has_popup: bool,
        area_width: u16,
        line_start: usize,
        selection: Option<(usize, usize)>,
    ) -> Line<'static> {
        if line.r#type == ConsoleMessageLineType::User {
            return Self::user_input_log_line(&line.text, background, accent, text_color, area_width, line_start, selection);
        }
        if line.r#type == ConsoleMessageLineType::Info {
            return Self::entry_hint_line(&line.text, has_popup, line_start, selection);
        }
        Self::selectable_plain_line(&line.text, line_start, selection, Style::default().fg(text_color))
    }

    pub fn user_input_log_line(text: &str, background: Color, accent: Color, text_color: Color, area_width: u16, line_start: usize, selection: Option<(usize, usize)>) -> Line<'static> {
        if area_width == 0 {
            return Line::from("");
        }
        let available_width = area_width.saturating_sub(ConsoleMessageLine::BORDER_MARGIN as u16 * 2) as usize;
        let dark_text = fit_text_to_width(&format!("{}{}", " ".repeat(ConsoleMessageLine::TEXT_PADDING), text), available_width.saturating_sub(1));
        let mut spans = vec![Span::raw(" ".repeat(ConsoleMessageLine::BORDER_MARGIN)), Span::styled(ACCENT_BAR, Style::default().fg(accent))];
        spans.extend(Self::selectable_spans(
            &dark_text,
            line_start,
            ConsoleMessageLine::TEXT_PADDING,
            text.chars().count(),
            selection,
            Style::default().fg(text_color).bg(background),
        ));
        Line::from(spans)
    }

    fn entry_hint_line(line: &str, has_popup: bool, line_start: usize, selection: Option<(usize, usize)>) -> Line<'static> {
        let text = format!("{}{}", " ".repeat(ConsoleMessageLine::TEXT_PADDING), line);
        let accent = style::muted_accent_color(has_popup);
        let mut spans = vec![Span::raw(" ".repeat(ConsoleMessageLine::BORDER_MARGIN)), Span::styled(ACCENT_BAR, Style::default().fg(accent))];
        spans.extend(Self::selectable_spans(
            &text,
            line_start,
            ConsoleMessageLine::TEXT_PADDING,
            line.chars().count(),
            selection,
            Style::default().fg(style::normal_text_color(has_popup)),
        ));
        Line::from(spans)
    }

    fn selectable_spans(text: &str, line_start: usize, selectable_start: usize, selectable_len: usize, selection: Option<(usize, usize)>, base_style: Style) -> Vec<Span<'static>> {
        text.chars()
            .enumerate()
            .map(|(offset, ch)| {
                let selectable_offset = offset.checked_sub(selectable_start).filter(|offset| *offset < selectable_len);
                let item_style = if selectable_offset.is_some_and(|offset| selection.is_some_and(|(start, end)| line_start + offset >= start && line_start + offset < end)) {
                    style::selected_text_style(base_style.fg.unwrap_or(Color::White))
                } else {
                    base_style
                };
                Span::styled(ch.to_string(), item_style)
            })
            .collect()
    }

    pub fn selectable_plain_line(line: &str, line_start: usize, selection: Option<(usize, usize)>, base_style: Style) -> Line<'static> {
        let text_start_column = ConsoleMessageLine::BORDER_MARGIN + ConsoleMessageLine::TEXT_PADDING + 1;
        let text = format!("{}{}", " ".repeat(text_start_column), line);
        Line::from(Self::selectable_spans(&text, line_start, text_start_column, line.chars().count(), selection, base_style))
    }

    // Event

    pub fn copy_selection(&mut self, content_width: u16) {
        let Some(selection) = self.selection else {
            return;
        };
        let (start, end) = selection.range();
        if start == end {
            return;
        }
        let text = self.messages_plain_text(content_width);
        let selected: String = text.chars().skip(start).take(end - start).collect();
        if selected.is_empty() {
            return;
        }
        let _ = Clipboard::new().and_then(|mut clipboard| clipboard.set_text(selected));
    }

    pub fn mouse_event(&mut self, mouse: MouseEvent, area: Rect) -> bool {
        if !popup::rect_contains(area, mouse.column, mouse.row) {
            return false;
        }
        self.hovered_group = self.row(mouse.column, mouse.row, area).and_then(|row| self.user_message_id_at(row, area.width));
        let index = self.char_index(mouse.column, mouse.row, area);
        match mouse.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                let Some(index) = index else { return false };
                self.begin_selection(index);
                true
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                let Some(index) = index else { return false };
                self.update_selection(index);
                true
            }
            MouseEventKind::ScrollUp => {
                self.scroll_to(-Self::MOUSE_SCROLL_LINES, area);
                true
            }
            MouseEventKind::ScrollDown => {
                self.scroll_to(Self::MOUSE_SCROLL_LINES, area);
                true
            }
            _ => false,
        }
    }

    pub fn begin_selection(&mut self, index: usize) {
        self.selection = Some(TextSelection::new(index));
    }

    pub fn update_selection(&mut self, index: usize) {
        if let Some(selection) = &mut self.selection {
            selection.focus = index;
        } else {
            self.selection = Some(TextSelection::new(index));
        }
    }

    pub fn char_index(&mut self, mouse_column: u16, mouse_row: u16, log_area: Rect) -> Option<usize> {
        let row = self.row(mouse_column, mouse_row, log_area)?;
        let column = mouse_column.saturating_sub(log_area.x) as usize;
        Some(self.char_index_at(row, column, log_area.width))
    }

    pub fn char_index_at(&mut self, row: usize, column: usize, content_width: u16) -> usize {
        let lines = self.messages_visual_lines(content_width);
        let mut index = 0usize;
        for (line_index, line) in lines.iter().enumerate() {
            if line_index == row {
                return index + line.selectable_index_at_column(column);
            }
            index += line.selectable_len();
            if line_index + 1 < lines.len() {
                index += 1;
            }
        }
        self.messages_plain_text(content_width).chars().count()
    }

    pub fn row(&mut self, mouse_column: u16, mouse_row: u16, area: Rect) -> Option<usize> {
        if !popup::rect_contains(area, mouse_column, mouse_row) {
            return None;
        }
        Some(mouse_row.saturating_sub(area.y) as usize + self.message_lines_scroll_pos(area) as usize)
    }

    pub fn user_message_id_at(&mut self, row: usize, content_width: u16) -> Option<usize> {
        self.messages_visual_lines(content_width)
            .get(row)
            .filter(|line| line.r#type == ConsoleMessageLineType::User)
            .map(|line| line.id)
    }

    pub fn scroll_to(&mut self, lines: i32, area: Rect) {
        let max_scroll = self.message_lines_height(area);
        self.scroll = if self.follow_tail { max_scroll } else { self.scroll.min(max_scroll) };
        if lines < 0 {
            self.scroll = self.scroll.saturating_sub(lines.unsigned_abs() as u16);
        } else {
            self.scroll = self.scroll.saturating_add(lines as u16).min(max_scroll);
        }
        self.follow_tail = self.scroll >= max_scroll;
    }
}
