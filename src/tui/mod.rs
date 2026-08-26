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

pub fn start_tui(strict_accents: bool) -> Result<()> {
    let exercises_dir = Path::new("exercises");
    let exercises = find_all_exercises(exercises_dir).unwrap_or_default();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new(exercises, strict_accents);
    let res = run_tui_app(&mut terminal, app);

    // Clean up and restore terminal state safely
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}
