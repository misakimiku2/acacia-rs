use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::event::{EnableFocusChange, EnableMouseCapture, Event, KeyEventKind};
use crossterm::execute;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::Block;

use super::components::style;
use crate::ui::pages::acacia_rs::AcaciaRsPopupPage;
use crate::ui::pages::console::ConsolePage;
use crate::ui::pages::exit::ExitPopupPage;
use crate::ui::pages::game_server_log::GameServerLogPopupPage;
use crate::ui::pages::game_server_logs::GameServerLogsPage;

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
    success_close_at: Option<Instant>,

    pub page: Page,
    pub popup_page: PopupPage,

    pub console_page: ConsolePage,
    pub game_server_logs_page: GameServerLogsPage,

    pub acacia_rs_popup_page: AcaciaRsPopupPage,
    pub exit_popup_page: ExitPopupPage,
    pub game_server_log_popup_page: GameServerLogPopupPage,
}

impl PageManager {
    pub fn new() -> Self {
        Self {
            success_close_at: None,
            page: Page::GameServerLogs,
            popup_page: PopupPage::AcaciaRs,
            console_page: ConsolePage::new(),
            game_server_logs_page: GameServerLogsPage::new(),
            acacia_rs_popup_page: AcaciaRsPopupPage::new(),
            exit_popup_page: ExitPopupPage::new(),
            game_server_log_popup_page: GameServerLogPopupPage::new(),
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        self.render_global_background(frame, area);
        // let content = content_area(area);

        match self.page {
            Page::Console => self.console_page.render(frame, area, self.has_popup()),
            Page::GameServerLogs => self.game_server_logs_page.reader(frame, area, self.has_popup()),
        };

        match self.popup_page {
            PopupPage::None => (),
            PopupPage::AcaciaRs => self.acacia_rs_popup_page.render(frame, area),
            PopupPage::Exit => self.exit_popup_page.render(frame, area),
            PopupPage::GameServerLog => self.game_server_log_popup_page.render(frame, area),
        };
    }

    pub fn render_global_background(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Block::default().style(Style::default().bg(style::global_background(self.has_popup()))), area);
    }

    pub fn event(&mut self, event: Event, area: Rect) {
        match event {
            Event::FocusGained => (),
            Event::FocusLost => {
                let _ = execute!(io::stdout(), EnableMouseCapture, EnableFocusChange); // Fix mouse capture after link click
            }
            Event::Key(key) => {
                if key.kind != KeyEventKind::Press {
                    return;
                }
            }
            Event::Mouse(_) => (),
            Event::Paste(_) => (),
            _ => {
                return;
            }
        }

        if self.popup_page != PopupPage::None {
            match self.popup_page {
                PopupPage::None => (),
                PopupPage::AcaciaRs => {
                    self.acacia_rs_popup_page.event(event, area, &mut self.popup_page);
                }
                PopupPage::Exit => self.exit_popup_page.event(event, area, &mut self.popup_page),
                PopupPage::GameServerLog => self.game_server_log_popup_page.event(event, area, &mut self.popup_page, &self.game_server_logs_page.logs),
            };
        } else {
            match self.page {
                Page::Console => self.console_page.event(event, area, &mut self.page, &mut self.popup_page),
                Page::GameServerLogs => {
                    self.game_server_logs_page
                        .event(event, area, &mut self.page, &mut self.popup_page, &mut self.game_server_log_popup_page);
                }
            }
        }
        return;
    }

    pub fn has_popup(&self) -> bool {
        self.popup_page != PopupPage::None
    }

    pub fn on_success(&mut self) {
        if self.popup_page == PopupPage::AcaciaRs {
            let success_close_at = self.success_close_at.get_or_insert_with(|| Instant::now() + Duration::from_millis(300)); // WIP to terminal
            if Instant::now() >= *success_close_at {
                self.success_close_at = None;
                self.popup_page = PopupPage::None;
                self.page = Page::Console;
            }
        }
    }

    pub fn on_error(&mut self) {
        if self.popup_page == PopupPage::AcaciaRs {
            self.popup_page = PopupPage::None;
        }
    }
}
