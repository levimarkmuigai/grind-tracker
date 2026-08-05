use ratatui::{Frame, crossterm::event, widgets::TableState};

use crate::{
    server::actors::DashboardCard,
    tui::{handler::AppAction, terminal::Tui, theme::Theme},
};

pub mod handler;
pub mod seeder;
pub mod terminal;
pub mod theme;
pub mod ui;

pub struct AppState {
    pub data: Vec<DashboardCard>,
    pub table_state: TableState,
    pub should_quit: bool,
}

impl AppState {
    pub fn new(data: Vec<DashboardCard>) -> Self {
        let mut table = TableState::default();

        Self {
            data,
            table_state: table,
            should_quit: false,
        }
    }

    pub fn update(&mut self, action: AppAction) {
        match action {
            AppAction::Quit => self.should_quit = true,
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
