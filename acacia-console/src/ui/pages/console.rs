use std::rc::Rc;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};

mod command;
mod composer;
mod conversation;
use composer::ConsoleComposerLayout;
use conversation::ConsoleConversationLayout;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::command::command_base::CommandItemAction;
use crate::ui::components::{popup, style};
use crate::ui::pages::console::command::ConsoleCommandLayout;
use crate::ui::pages::console::conversation::ConsoleMessage;
use crate::ui::pages::{Page, PopupPage};

enum ConsoleLayout {
    Content,
    Input,
}

pub struct ConsolePage {
    pub console_conversation: ConsoleConversationLayout,
    pub console_command: ConsoleCommandLayout,
    pub console_composer: ConsoleComposerLayout,
    active_layout: Option<ConsoleLayout>,
}

impl ConsolePage {
    const MAX_MENU_HEIGHT: u16 = 10;

    pub fn new() -> Self {
        Self {
            console_conversation: ConsoleConversationLayout::new(),
            console_command: ConsoleCommandLayout::new(),
            console_composer: ConsoleComposerLayout::new(),
            active_layout: Some(ConsoleLayout::Input),
        }
    }

    pub fn area_chunks(&self, area: Rect) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(5),                                     // Content
                Constraint::Length(1),                                  // Margin
                Constraint::Length(self.console_composer.height(area)), // Input
                Constraint::Length(2),                                  // Footer
            ])
            .split(area)
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, has_popup: bool) {
        let chunks = self.area_chunks(area);
        self.console_conversation.render(frame, chunks[0], has_popup);
        self.console_composer.render(frame, chunks[2], has_popup);
        if let Some(command_area) = self.command_area(chunks[2]) {
            self.console_command.render(frame, command_area, has_popup);
        }
        self.render_footer(frame, chunks[3], has_popup)
    }

    pub fn render_footer(&self, frame: &mut Frame, area: Rect, has_popup: bool) {
        let normal = style::normal_text_color(has_popup);
        let muted = style::muted_text_color(has_popup);
        let lines = vec![
            Line::from(vec![
                Span::styled("ctrl+l", Style::default().fg(normal)),
                Span::styled(" ", Style::default()),
                Span::styled("server-logs", Style::default().fg(muted)),
                Span::styled("  ", Style::default()),
            ]),
            Line::from(""),
        ];
        frame.render_widget(Paragraph::new(lines).alignment(Alignment::Right), area);
    }

    pub fn menu_height(&self) -> u16 {
        if self.console_command.is_show() {
            return self.console_command.item_height().min(Self::MAX_MENU_HEIGHT);
        }
        0
    }

    fn command_area(&self, input_area: Rect) -> Option<Rect> {
        let height = self.menu_height().min(input_area.y);
        (height > 0).then_some(Rect {
            x: input_area.x,
            y: input_area.y - height,
            width: input_area.width,
            height,
        })
    }

    pub fn event(&mut self, event: Event, area: Rect, current_page: &mut Page, current_popup_page: &mut PopupPage) {
        match event {
            Event::Key(key) => self.key_event(key, area, current_page, current_popup_page),
            Event::Mouse(mouse) => self.mouse_event(mouse, area),
            Event::Paste(text) => {
                self.console_composer.input.replace_selection(&text);
            }
            Event::FocusGained => {
                self.console_composer.input.cursor_visible = true;
            }
            Event::FocusLost => {
                self.console_composer.input.cursor_visible = false;
            }
            _ => (),
        }
    }

    fn key_event(&mut self, key: KeyEvent, area: Rect, current_page: &mut Page, current_popup_page: &mut PopupPage) {
        let result = match key.code {
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => match self.active_layout {
                Some(ConsoleLayout::Content) => {
                    self.console_conversation.copy_selection(area.width);
                    true
                }
                _ => false,
            },
            KeyCode::Char('x') if key.modifiers.contains(KeyModifiers::CONTROL) => match self.active_layout {
                Some(ConsoleLayout::Content) => true,
                _ => false,
            },
            KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                *current_page = Page::GameServerLogs;
                true
            }
            KeyCode::Enter => {
                if self.apply_selected_command() {
                } else if !self.console_composer.input.text.is_empty() {
                    self.commit_input();
                }
                true
            }
            KeyCode::Tab => self.apply_selected_command(),
            KeyCode::Esc => {
                *current_popup_page = PopupPage::Exit;
                true
            }
            KeyCode::Down => {
                if self.console_command.is_show() {
                    self.console_command.move_command_down(self.menu_height() as usize);
                    true
                } else {
                    false
                }
            }
            KeyCode::Up => {
                if self.console_command.is_show() {
                    self.console_command.move_command_up(self.menu_height() as usize);
                    true
                } else if self.console_composer.input.text.is_empty() {
                    // self.enter_history_mode();
                    true
                } else {
                    false
                }
            }
            _ => false,
        };
        if !result {
            self.console_composer.key_event(key, area);
        }
        self.console_command.update_items(&self.console_composer.input.text);
    }

    fn mouse_event(&mut self, mouse: MouseEvent, area: Rect) {
        let chunks = self.area_chunks(area);
        let command_area = self.command_area(chunks[2]);

        if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
            if command_area.is_some_and(|area| popup::rect_contains(area, mouse.column, mouse.row)) {
                self.active_layout = Some(ConsoleLayout::Input);
            } else if popup::rect_contains(chunks[2], mouse.column, mouse.row) {
                self.active_layout = Some(ConsoleLayout::Input);
            } else if popup::rect_contains(chunks[0], mouse.column, mouse.row) {
                self.active_layout = Some(ConsoleLayout::Content);
            } else {
                self.active_layout = None;
            }
        }

        if let Some(command_area) = command_area {
            if self.console_command.mouse_event(mouse, command_area) {
                if mouse.kind == MouseEventKind::Up(MouseButton::Left) {
                    if self.apply_selected_command() {
                        self.console_command.update_items(&self.console_composer.input.text);
                    }
                }
                return;
            }
        }

        if self.console_conversation.mouse_event(mouse, chunks[0]) {
            return;
        }

        if popup::rect_contains(chunks[2], mouse.column, mouse.row) {
            self.console_composer.mouse_event(mouse, chunks[2]);
        }
    }

    fn commit_input(&mut self) {
        let text = self.console_composer.input.take_text();
        let message = ConsoleMessage::User(text.clone());
        self.console_conversation.messages.push(message);
        let result: String = self.console_command.command_manager.execute(&text);
        let message = ConsoleMessage::Ret(result);
        self.console_conversation.messages.push(message);
    }

    fn apply_selected_command(&mut self) -> bool {
        let Some(action) = self.console_command.selected_action() else {
            return false;
        };

        match action {
            CommandItemAction::None => false,
            CommandItemAction::Insert(text) => {
                self.console_composer.input.text = text;
                self.console_composer.input.cursor = self.console_composer.input.text.chars().count();
                self.console_composer.input.clear_selection();
                self.console_composer.input.reset_cursor_blink();
                true
            }
            CommandItemAction::Submit(text) => {
                self.console_composer.input.text = text;
                self.console_composer.input.cursor = self.console_composer.input.text.chars().count();
                self.console_composer.input.clear_selection();
                self.commit_input();
                true
            }
        }
    }
}
