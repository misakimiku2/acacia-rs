use crossterm::event::{self, DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste, EnableFocusChange, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::sync::mpsc::TryRecvError;
use std::time::{Duration, Instant};
use std::{io, panic};

use crate::base::game_server::game_server_base::{GameServerEvent, GameServerMessage};
use crate::base::game_server::game_server_manager::game_server_manager;
use crate::ui::pages::PageManager;

const TERMINAL_TICK_RATE: Duration = Duration::from_millis(16);
const STARTUP_READY_LOGS: [&str; 3] = ["GameServer listening", "ApiServer listening", "GMServer listening"];

pub fn terminal_main() -> color_eyre::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture, EnableBracketedPaste, EnableFocusChange)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    game_server_manager().lock().unwrap().start()?;
    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| terminal_panic(&mut terminal)));
    let stop_result = game_server_manager().lock().unwrap().stop();

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), DisableFocusChange, DisableBracketedPaste, DisableMouseCapture, LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    stop_result?;

    match result {
        Ok(result) => result,
        Err(payload) => panic::resume_unwind(payload),
    }
}

fn terminal_panic(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> color_eyre::Result<()> {
    let mut page = PageManager::new();
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|frame| page.render(frame))?;
        let timeout = TERMINAL_TICK_RATE.checked_sub(last_tick.elapsed()).unwrap_or(Duration::ZERO);
        if event::poll(timeout)? {
            loop {
                page.event(event::read()?, terminal.size()?.into());
                if page.exit_popup_page.exit_flag {
                    return Ok(());
                }

                if !event::poll(Duration::ZERO)? {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= TERMINAL_TICK_RATE {
            // app.tick();
            drain_game_server_logs(&mut page);
            last_tick = Instant::now();
        }
    }
}

fn drain_game_server_logs(page: &mut PageManager) {
    let mut startup_ready_logs = [false; STARTUP_READY_LOGS.len()];
    let mut has_rust_error = false;
    for (_, log) in &page.game_server_logs_page.logs {
        mark_startup_ready_log(log, &mut startup_ready_logs);
    }

    loop {
        let message = game_server_manager().lock().unwrap().try_recv();
        match message {
            Ok(message) => {
                if is_rust_error_log(&message) {
                    has_rust_error = true;
                }
                mark_startup_ready_log(&message.1, &mut startup_ready_logs);
                page.game_server_logs_page.logs.push(message);
            }
            Err(TryRecvError::Empty | TryRecvError::Disconnected) => break,
        }
    }

    if has_rust_error {
        page.on_error();
    } else if startup_ready_logs.iter().all(|ready| *ready) {
        page.on_success();
    }
}

fn mark_startup_ready_log(log: &str, startup_ready_logs: &mut [bool; STARTUP_READY_LOGS.len()]) {
    for (index, startup_log) in STARTUP_READY_LOGS.iter().enumerate() {
        if log.contains(startup_log) {
            startup_ready_logs[index] = true;
        }
    }
}

fn is_rust_error_log((event, _): &GameServerMessage) -> bool {
    matches!(event, GameServerEvent::StdError | GameServerEvent::InError)
}
