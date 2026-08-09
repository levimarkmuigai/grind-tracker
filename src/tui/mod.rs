use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyEventKind},
    widgets::TableState,
};

use crate::tui::{
    client::{Stats, TableData},
    handler::AppAction,
    terminal::Tui,
    theme::Theme,
};

pub mod client;
pub mod handler;
pub mod terminal;
pub mod theme;
pub mod ui;

#[derive(Debug, Clone, Copy)]
pub enum AppMode {
    Dashboard,
    Review,
}

pub struct AppState {
    pub mode: AppMode,
    pub table_data: Vec<TableData>,
    pub stats: Stats,
    pub table_state: TableState,
    pub selected_problem: Option<TableData>,
    pub should_quit: bool,
}

impl AppState {
    pub fn new(table_data: Vec<TableData>, stats: Stats) -> Self {
        let mut table = TableState::default();

        let mode = AppMode::Dashboard;

        let selected_problem = None;

        if !table_data.is_empty() {
            table.select(Some(0));
        }

        Self {
            mode,
            table_data,
            stats,
            table_state: table,
            selected_problem,
            should_quit: false,
        }
    }

    pub fn update(&mut self, action: AppAction) {
        match action {
            AppAction::Quit => self.should_quit = true,
            AppAction::SelectUp => self.table_state.select_previous(),
            AppAction::SelectDown => self.table_state.select_next(),
            AppAction::OpenReview => {
                if let Some(i) = self.table_state.selected() {
                    self.selected_problem = self.table_data.get(i).cloned();
                    self.mode = AppMode::Review;
                }
            }
            AppAction::SubmitRating(_rating) => {
                //TODO: Post /review { problem_id, rating }
                self.mode = AppMode::Dashboard;
                self.selected_problem = None;
            }
            AppAction::CloseReview => {
                self.mode = AppMode::Dashboard;
                self.selected_problem = None;
            }
            AppAction::None => (),
        }
    }
}

fn render(frame: &mut Frame, state: &mut AppState, theme: &Theme) {
    match state.mode {
        AppMode::Dashboard => {
            let (stats, table, footer) = ui::area(frame.area());

            ui::stats::render_stats(frame, stats, state, theme);
            ui::table::render_table(frame, table, state, theme);
            ui::footer::render_footer(frame, footer, state, theme);
        }

        AppMode::Review => {
            if let Some(problem) = &state.selected_problem {
                ui::review::render_review(frame, frame.area(), problem, theme);
            }
        }
    }
}

pub fn run(mut state: AppState, terminal: &mut Tui, theme: &Theme) -> color_eyre::Result<()> {
    while !state.should_quit {
        terminal.draw(|frame| render(frame, &mut state, theme))?;

        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                let action = handler::map_event(key, &state.mode);
                state.update(action);
            }
            _ => {}
        }
    }
    Ok(())
}
