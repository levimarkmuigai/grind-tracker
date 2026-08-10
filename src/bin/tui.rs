use std::sync::mpsc;

use grind_tracker::tui::{self, client::Api, terminal};

fn main() -> color_eyre::Result<()> {
    let api = Api::new()?;

    let (table_data, stats) = api.seed_dash_data()?;

    let (action_tx, action_rx) = mpsc::channel::<tui::handler::AppAction>();

    let state = tui::AppState::new(table_data, stats, api, action_tx);

    let mut terminal = terminal::init()?;

    let theme = tui::theme::Theme::default();

    tui::run(state, &mut terminal, &theme, action_rx)?;

    ratatui::restore();

    Ok(())
}
