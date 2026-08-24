use ratatui::layout::{Constraint, Layout, Rect};

pub mod footer;
pub mod review;
pub mod stats;
pub mod table;
pub mod toast;

pub fn area(area: Rect) -> (Rect, Rect, Rect) {
    let [stats, table, footer] = Layout::vertical([
        Constraint::Length(10),
        Constraint::Fill(1),
        Constraint::Length(2),
    ])
    .areas(area);

    (stats, table, footer)
}
