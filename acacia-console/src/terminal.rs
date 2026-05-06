use crossterm::event::{self, DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste, EnableFocusChange, EnableMouseCapture, Event, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::time::{Duration, Instant};
use std::{io, panic};

use crate::base::game_server::game_server_manager::game_server_manager;
use crate::ui::pages::PageManager;

const TERMINAL_TICK_RATE: Duration = Duration::from_millis(16);

pub fn terminal_main() -> color_eyre::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture, EnableBracketedPaste, EnableFocusChange)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    // let game_server_available = game_server_manager().lock().unwrap().start().is_ok();
    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| terminal_panic(&mut terminal)));

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), DisableFocusChange, DisableBracketedPaste, DisableMouseCapture, LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    match result {
        Ok(result) => result,
        Err(payload) => panic::resume_unwind(payload),
    }
}

fn terminal_panic(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> color_eyre::Result<()> {
    let mut page = PageManager::new();
    // let mut app = App::new();
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|frame| page.reader(frame))?;
        let timeout = TERMINAL_TICK_RATE.checked_sub(last_tick.elapsed()).unwrap_or(Duration::ZERO);
        if event::poll(timeout)? {
            loop {
                if terminal_event(&mut page, event::read()?, terminal.size()?.into()) {
                    return Ok(());
                }

                if !event::poll(Duration::ZERO)? {
                    break;
                }
            }
        }

        // 到达 tick 间隔后更新时间戳。本 demo 没有动画或后台任务，所以 tick 只用于保持循环节奏。
        if last_tick.elapsed() >= TERMINAL_TICK_RATE {
            // app.tick();
            // app.drain_program_events();
            last_tick = Instant::now();
        }
    }
}

fn terminal_event(page: &mut PageManager, event: Event, area: ratatui::layout::Rect) -> bool {
    match event {
        Event::Key(key) if key.kind == KeyEventKind::Press => page.event(Event::Key(key), area),
        Event::Mouse(mouse) => page.event(Event::Mouse(mouse), area),
        Event::Paste(text) => {
            page.insert_text(&text);
            false
        }
        Event::FocusGained => {
            let _ = execute!(io::stdout(), EnableMouseCapture, EnableFocusChange);
            page.set_window_focused(true);
            false
        }
        Event::FocusLost => {
            page.set_window_focused(false);
            false
        }
        _ => false,
    }
}
