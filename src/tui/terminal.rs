use std::io::{self, stdout};

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    crossterm::{
        ExecutableCommand,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};

pub type Tui = Terminal<CrosstermBackend<std::io::Stdout>>;

pub fn init() -> io::Result<Tui> {
    enable_raw_mode()?;
    let mut stdout = stdout();

    stdout.execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);

    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}

pub fn restore() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}
