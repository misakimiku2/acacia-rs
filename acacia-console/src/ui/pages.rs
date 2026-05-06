use std::process::exit;

use crossterm::event::Event;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::Block;

use super::components::style;
use crate::ui::pages::console::ConsolePage;

mod acacia_rs;
mod console;
mod exit;
mod game_server_log;
mod game_server_logs;

#[derive(PartialEq)]
pub enum Page {
    Console,
    GameServerLogs,
}

#[derive(PartialEq)]
pub enum PopupPage {
    None,
    AcaciaRs,
    Exit,
    GameServerLog,
}

pub struct PageManager {
    pub page: Page,
    pub popup_page: PopupPage,
    pub console_page: ConsolePage,
}

impl PageManager {
    pub fn new() -> Self {
        Self {
            page: Page::GameServerLogs,
            popup_page: PopupPage::AcaciaRs,
            console_page: ConsolePage::new(),
        }
    }

    pub fn reader(&self, frame: &mut Frame) {
        let area = frame.area();
        self.reader_global_background(frame, area);
        // let content = content_area(area);

        match self.page {
            Page::Console => self.console_page.reader(frame, area),
            Page::GameServerLogs => (), // game_server_logs::reader(frame, app, area),
        };

        match self.popup_page {
            PopupPage::None => (),
            PopupPage::AcaciaRs => (),      // acacia_rs::reader(frame, app, area),
            PopupPage::Exit => (),          // exit::reader(frame, app, area),
            PopupPage::GameServerLog => (), // game_server_log::reader(frame, app, area),
        };
    }

    pub fn reader_global_background(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Block::default().style(Style::default().bg(style::global_background(self.has_popup()))), area);
    }

    pub fn event(&mut self, event: Event, area: Rect) -> bool {
        false
    }

    pub fn insert_text(&mut self, text: &str) {
        if self.page == Page::Console && self.popup_page == PopupPage::None {
            // self.console_page.insert_text(text);
        }
    }

    pub fn set_window_focused(&mut self, focused: bool) {
        // self.console_page.set_window_focused(focused);
    }

    pub fn has_popup(&self) -> bool {
        self.popup_page != PopupPage::None
    }
}
