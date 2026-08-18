use std::sync::mpsc::{Receiver, Sender};

use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyEventKind},
    widgets::TableState,
};

use crate::tui::{
    client::{Api, Stats, TableData},
    handler::{AppAction, ToastKind},
    terminal::Tui,
    theme::Theme,
    toast::Toast,
};

pub mod client;
pub mod handler;
pub mod terminal;
pub mod theme;
pub mod toast;
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
    pub api: Api,
    pub toast: Option<Toast>,
    pub action_tx: Sender<AppAction>,
}

impl AppState {
    pub fn new(
        table_data: Vec<TableData>,
        stats: Stats,
        api: Api,
        action_tx: Sender<AppAction>,
    ) -> Self {
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
            api,
            toast: None,
            action_tx,
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

            AppAction::CloseReview => {
                self.mode = AppMode::Dashboard;
                self.selected_problem = None;
            }

            AppAction::SubmitRating(review) => {
                if let Some(problem) = &self.selected_problem {
                    self.toast = Some(Toast::new("submitting...", ToastKind::Info, 10));

                    let api = self.api.clone();
                    let tx = self.action_tx.clone();
                    let problem_id = problem.problem_id;

                    std::thread::spawn(move || match api.submit_review(problem_id, review) {
                        Ok(()) => {
                            let _ = tx.send(AppAction::ReviewSubmitted);
                            let _ = tx.send(AppAction::ShowToast {
                                message: "review submitted!".into(),
                                kind: ToastKind::Success,
                            });

                            let _ = tx.send(AppAction::RefreshData);
                        }
                        Err(e) => {
                            let _ = tx.send(AppAction::ShowToast {
                                message: format!("failed: {e}"),
                                kind: ToastKind::Error,
                            });
                        }
                    });
                }
            }

            AppAction::ReviewSubmitted => {
                self.mode = AppMode::Dashboard;
                self.selected_problem = None;
            }

            AppAction::RefreshData => {
                let api = self.api.clone();
                let tx = self.action_tx.clone();

                std::thread::spawn(move || match api.seed_dash_data() {
                    Ok((table_data, stats)) => {
                        let _ = tx.send(AppAction::RefreshedData { table_data, stats });
                    }
                    Err(e) => {
                        let _ = tx.send(AppAction::ShowToast {
                            message: format!("refresh failed: {e}"),
                            kind: ToastKind::Error,
                        });
                    }
                });
            }

            AppAction::RefreshedData { table_data, stats } => {
                self.table_data = table_data;
                self.stats = stats;

                if !self.table_data.is_empty() {
                    self.table_state.select(Some(0));
                }
            }

            AppAction::ShowToast { message, kind } => {
                self.toast = Some(Toast::new(message, kind, 3));
            }

            AppAction::ClearToast => {
                self.toast = None;
            }

            AppAction::None => {}
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

    if let Some(toast) = &state.toast
        && !toast.is_expired()
    {
        ui::toast::render_toast(frame, toast, theme);
    }
}

pub fn run(
    mut state: AppState,
    terminal: &mut Tui,
    theme: &Theme,
    action_rx: Receiver<AppAction>,
) -> color_eyre::Result<()> {
    while !state.should_quit {
        terminal.draw(|frame| render(frame, &mut state, theme))?;

        while let Ok(action) = action_rx.try_recv() {
            state.update(action);
        }

        if let Some(toast) = &state.toast
            && toast.is_expired()
        {
            state.toast = None;
        }

        if event::poll(std::time::Duration::from_millis(50))?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            let action = handler::map_event(key, &state.mode);
            state.update(action);
        }
    }
    Ok(())
}
