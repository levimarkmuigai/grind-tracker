use grind_tracker::tui::{self, seeder, terminal};

fn main() -> color_eyre::Result<()> {
    let (table_data, stats) = seeder::seed_dash_data()?;

    let state = tui::AppState::new(table_data, stats);

    let mut terminal = terminal::init()?;

    let theme = tui::theme::Theme::default();

    tui::run(state, &mut terminal, &theme)?;

    ratatui::restore();

    Ok(())
}
