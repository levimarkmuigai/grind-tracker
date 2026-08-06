use grind_tracker::{
    server::actors::TableData,
    tui::{self, terminal},
};

fn main() -> color_eyre::Result<()> {
    let data: Vec<TableData> = vec![];

    let state = tui::AppState::new(data);

    let mut terminal = terminal::init()?;

    let theme = tui::theme::Theme::default();

    tui::run(state, &mut terminal, &theme)?;

    ratatui::restore();

    Ok(())
}
