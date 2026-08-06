use ratatui::{Frame, crossterm::event, widgets::TableState};

use crate::{
    server::actors::{Stats, TableData},
    tui::{handler::AppAction, terminal::Tui, theme::Theme},
};

pub mod handler;
pub mod seeder;
pub mod terminal;
pub mod theme;
pub mod ui;

pub struct AppState {
    pub table_data: Vec<TableData>,
    pub cards_data: Stats,
    pub table_state: TableState,
    pub should_quit: bool,
}

impl AppState {
    pub fn new(table_data: Vec<TableData>, cards_data: Stats) -> Self {
        let mut table = TableState::default();

        if !table_data.is_empty() {
            table.select(Some(0));
        }

        Self {
            table_data,
            cards_data,
            table_state: table,
            should_quit: false,
        }
    }

    pub fn update(&mut self, action: AppAction) {
        match action {
            AppAction::Quit => self.should_quit = true,
            AppAction::SelectUp => self.table_state.select_previous(),
            AppAction::SelectDown => self.table_state.select_next(),
            AppAction::None => (),
        }
    }
}

fn render(frame: &mut Frame, state: &mut AppState, theme: &Theme) {
    let (stats, table, footer) = ui::area(frame.area());

    ui::stats::render_stats(frame, stats, state, theme);
    ui::table::render_table(frame, table, state, theme);
    ui::footer::render_footer(frame, footer, state, theme);
}

pub fn run(mut state: AppState, terminal: &mut Tui, theme: &Theme) -> color_eyre::Result<()> {
    while !state.should_quit {
        terminal.draw(|frame| render(frame, &mut state, theme))?;

        let event = event::read()?;
        let action = handler::map_event(event);

        state.update(action);
    }
    Ok(())
}
