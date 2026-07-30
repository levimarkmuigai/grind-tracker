use ratatui::widgets::TableState;

use crate::server::actors::DashboardCard;

pub mod seeder;
pub mod theme;
pub mod ui;

pub struct AppState {
    pub data: Vec<DashboardCard>,
    pub table_state: TableState,
    pub should_quit: bool,
}
