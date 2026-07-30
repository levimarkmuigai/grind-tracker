use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders, Cell, Row, Table},
};

use crate::tui::{AppState, theme::Theme};

pub fn render_table(frame: &mut Frame, area: Rect, state: &mut AppState, theme: &Theme) {
    let header = ["id", "title", "diff", "topics", "state", "due"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(Style::new().bg(theme.highlight))
        .height(1);

    let rows = state.data.iter().map(|d| Row::new(d.as_array()).height(2));

    let widths = [
        Constraint::Length(4),
        Constraint::Min(0),
        Constraint::Length(7),
        Constraint::Min(0),
        Constraint::Length(8),
        Constraint::Length(9),
    ];

    let table = Table::new(rows, widths).header(header).block(
        Block::default()
            .borders(Borders::all())
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(theme.ink)),
    );

    frame.render_stateful_widget(table, area, &mut state.table_state);
}
