pub mod app;
pub mod events;
pub mod ui;

use crate::core::curriculum::find_all_exercises;
use crate::tui::app::App;
use crate::tui::events::run_tui_app;
use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use std::path::Path;

struct RawModeGuard;

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        let _ = execute!(io::stdout(), crossterm::cursor::Show);
    }
}

pub fn start_tui(strict_accents: bool) -> Result<()> {
    let exercises_dir = Path::new("exercises");
    let exercises = find_all_exercises(exercises_dir).unwrap_or_default();

    // Register panic hook to restore terminal if a panic occurs
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        let _ = execute!(io::stdout(), crossterm::cursor::Show);
        default_panic(panic_info);
    }));

    enable_raw_mode()?;
    let _guard = RawModeGuard;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new(exercises, strict_accents);
    run_tui_app(&mut terminal, app)
}
