use crossterm::event::{MouseEvent, MouseEventKind};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, ListItem, Paragraph},
};
use unicode_width::UnicodeWidthStr;

use crate::{
    command::{
        command_base::{CommandItem, CommandItemAction},
        command_manager::CommandManager,
    },
    ui::components::{
        popup,
        style::{self, ACCENT_BAR},
    },
};

pub struct ConsoleCommandLayout {
    pub selected: Option<usize>,
    pub scroll_offset: usize,
    pub command_manager: CommandManager,
    pub command_items: Vec<CommandItem>,
}

impl ConsoleCommandLayout {
    const BORDER_MARGIN: u16 = 2;
    const TEXT_PADDING: u16 = 1;
    const MOUSE_SCROLL_LINES: i32 = 3;

    pub fn new() -> Self {
        Self {
            selected: None,
            scroll_offset: 0,
            command_manager: CommandManager::new(),
            command_items: Vec::new(),
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect, has_popup: bool) {
        let normal_background_color = style::block_backgrounds(has_popup).1;
        let selected_background_color = style::selected_item_background(has_popup);
        let selected_text_color = style::selected_item_text_color(has_popup);
        let normal_color: Color = style::normal_text_color(has_popup);
        let muted_color = style::muted_text_color(has_popup);

        let list_area = self.list_area(area);
        let items = &self.command_items;
        let title_width = items
            .iter()
            .filter_map(|item| match (item.name.as_deref(), item.desc.as_deref()) {
                (Some(title), Some(desc)) if !title.is_empty() && !desc.is_empty() => Some(title),
                _ => None,
            })
            .map(UnicodeWidthStr::width)
            .max()
            .unwrap_or(0)
            + 5;
        let list_items: Vec<ListItem> = items
            .iter()
            .skip(self.scroll_offset)
            .take(list_area.height as usize)
            .enumerate()
            .map(|(relative_index, item)| {
                let index = self.scroll_offset + relative_index;
                let selected = self.selected == Some(index);
                let line = Self::command_item_to_line(item, title_width, normal_color, muted_color, selected_text_color, selected_background_color, selected, list_area.width);
                if selected {
                    ListItem::new(line).style(Style::default().fg(selected_text_color).bg(selected_background_color))
                } else {
                    ListItem::new(line)
                }
            })
            .collect();
        let list = List::new(list_items).style(Style::default().bg(normal_background_color));
        frame.render_widget(Paragraph::new(self.render_background(area, has_popup)), area);
        frame.render_widget(list, list_area);
    }

    fn render_background(&self, area: Rect, has_popup: bool) -> Vec<Line<'static>> {
        let accent_color = style::command_popup_accent(has_popup);
        let normal_color = style::normal_text_color(has_popup);
        let (_, background_color) = style::block_backgrounds(has_popup);
        let background_style = Style::default().fg(normal_color).bg(background_color);

        (0..area.height).map(|_| Self::render_background_line(area.width, accent_color, background_style)).collect()
    }

    fn render_background_line(area_width: u16, accent_color: Color, background_style: Style) -> Line<'static> {
        let left_margin = " ".repeat(Self::BORDER_MARGIN as usize);
        let rest_width = area_width.saturating_sub(Self::BORDER_MARGIN * 2 + 1) as usize;
        let right_margin = " ".repeat(Self::BORDER_MARGIN as usize);
        Line::from(vec![
            Span::raw(left_margin),
            Span::styled(ACCENT_BAR, Style::default().fg(accent_color)),
            Span::styled(" ".repeat(rest_width), background_style),
            Span::raw(right_margin),
        ])
    }

    fn list_area(&self, area: Rect) -> Rect {
        Rect {
            x: area.x.saturating_add(Self::BORDER_MARGIN + 1),
            y: area.y,
            width: area.width.saturating_sub(Self::BORDER_MARGIN * 2 + 1).max(1),
            height: area.height,
        }
    }

    fn command_item_to_line(item: &CommandItem, title_width: usize, normal: Color, muted: Color, selected_text: Color, selected_background: Color, selected: bool, line_width: u16) -> Line<'static> {
        let title = item.name.as_deref().unwrap_or("");
        let desc = item.desc.as_deref().unwrap_or("");
        let title_style = Self::item_style(normal, selected_text, selected_background, selected);
        let desc_style = Self::item_style(muted, selected_text, selected_background, selected);
        let content = match (title.is_empty(), desc.is_empty()) {
            (false, false) => Line::from(vec![Span::styled(Self::padded_title(title, title_width), title_style), Span::styled(desc.to_string(), desc_style)]),
            (false, true) => Line::from(Span::styled(title.to_string(), title_style)),
            (true, false) => Line::from(Span::styled(desc.to_string(), desc_style)),
            (true, true) => Line::from(""),
        };
        if line_width <= Self::TEXT_PADDING * 2 {
            return content;
        }

        let padding_style = Self::item_style(normal, selected_text, selected_background, selected);
        let mut spans = Vec::with_capacity(content.spans.len() + 2);
        spans.push(Span::styled(" ".repeat(Self::TEXT_PADDING as usize), padding_style));
        spans.extend(content.spans);
        spans.push(Span::styled(" ".repeat(Self::TEXT_PADDING as usize), padding_style));
        Line::from(spans)
    }

    fn item_style(color: Color, selected_text: Color, selected_background: Color, selected: bool) -> Style {
        if selected {
            Style::default().fg(selected_text).bg(selected_background)
        } else {
            Style::default().fg(color)
        }
    }

    fn padded_title(title: &str, width: usize) -> String {
        let padding = width.saturating_sub(title.width());
        format!("{}{}", title, " ".repeat(padding))
    }

    fn is_selectable(item: &CommandItem) -> bool {
        item.action != CommandItemAction::None
    }

    fn first_selectable(&self) -> Option<usize> {
        self.command_items.iter().position(Self::is_selectable)
    }

    fn ensure_selected_visible(&mut self, visible_height: usize) {
        if visible_height == 0 || self.command_items.is_empty() {
            self.scroll_offset = 0;
            return;
        }

        let max_offset = self.command_items.len().saturating_sub(visible_height);
        self.scroll_offset = self.scroll_offset.min(max_offset);

        let Some(selected) = self.selected else {
            self.scroll_offset = 0;
            return;
        };

        if selected < self.scroll_offset {
            self.scroll_offset = selected;
        } else if selected >= self.scroll_offset + visible_height {
            self.scroll_offset = selected + 1 - visible_height;
        }
    }

    fn clamp_scroll_offset(&mut self, visible_height: usize) {
        let max_offset = self.command_items.len().saturating_sub(visible_height);
        self.scroll_offset = self.scroll_offset.min(max_offset);
    }

    fn row(&self, mouse_column: u16, mouse_row: u16, area: Rect) -> Option<usize> {
        let list_area = self.list_area(area);
        if !popup::rect_contains(list_area, mouse_column, mouse_row) {
            return None;
        }
        Some(mouse_row.saturating_sub(list_area.y) as usize + self.scroll_offset)
    }

    fn select_row(&mut self, row: usize) -> bool {
        let Some(item) = self.command_items.get(row) else {
            return false;
        };
        if !Self::is_selectable(item) {
            return false;
        }
        self.selected = Some(row);
        true
    }

    pub fn is_show(&self) -> bool {
        !self.command_items.is_empty()
    }

    pub fn item_height(&self) -> u16 {
        self.command_items.len() as u16
    }

    pub fn update_items(&mut self, input_text: &str) {
        self.command_items = self.command_manager.command(input_text);
        self.selected = match self.selected {
            Some(selected) if self.command_items.get(selected).is_some_and(Self::is_selectable) => Some(selected),
            _ => self.first_selectable(),
        };
        if self.selected.is_none() {
            self.scroll_offset = 0;
        } else if self.scroll_offset >= self.command_items.len() {
            self.scroll_offset = self.command_items.len().saturating_sub(1);
        }
    }

    pub fn move_command_down(&mut self, visible_height: usize) {
        self.move_command(visible_height, false);
    }

    pub fn move_command_up(&mut self, visible_height: usize) {
        self.move_command(visible_height, true);
    }

    pub fn selected_action(&self) -> Option<CommandItemAction> {
        self.selected
            .and_then(|selected| self.command_items.get(selected))
            .filter(|item| Self::is_selectable(item))
            .map(|item| item.action.clone())
    }

    fn move_command(&mut self, visible_height: usize, reverse: bool) {
        let Some(selected) = self.selected else {
            self.selected = self.first_selectable();
            self.ensure_selected_visible(visible_height);
            return;
        };
        if self.command_items.is_empty() {
            return;
        }

        if let Some(index) = self.selectable_index_after(selected, reverse) {
            self.selected = Some(index);
            self.ensure_selected_visible(visible_height);
        } else {
            self.selected = None;
        }
    }

    fn selectable_index_after(&self, selected: usize, reverse: bool) -> Option<usize> {
        let len = self.command_items.len();
        (1..=len)
            .map(|offset| if reverse { (selected + len - offset) % len } else { (selected + offset) % len })
            .find(|index| Self::is_selectable(&self.command_items[*index]))
    }

    pub fn mouse_event(&mut self, mouse: MouseEvent, area: Rect) -> bool {
        let list_area = self.list_area(area);
        if !popup::rect_contains(list_area, mouse.column, mouse.row) {
            return false;
        }

        match mouse.kind {
            MouseEventKind::ScrollUp => {
                self.scroll_to(-Self::MOUSE_SCROLL_LINES, list_area.height as usize);
                true
            }
            MouseEventKind::ScrollDown => {
                self.scroll_to(Self::MOUSE_SCROLL_LINES, list_area.height as usize);
                true
            }
            _ => self.row(mouse.column, mouse.row, area).is_some_and(|row| self.select_row(row)),
        }
    }

    pub fn scroll_to(&mut self, lines: i32, visible_height: usize) {
        self.clamp_scroll_offset(visible_height);
        if lines < 0 {
            self.scroll_offset = self.scroll_offset.saturating_sub(lines.unsigned_abs() as usize);
        } else {
            let max_offset = self.command_items.len().saturating_sub(visible_height);
            self.scroll_offset = self.scroll_offset.saturating_add(lines as usize).min(max_offset);
        }
    }
}
