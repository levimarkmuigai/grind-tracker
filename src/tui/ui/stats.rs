use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders},
};

use crate::tui::{AppState, theme::Theme};

pub fn render_stats(frame: &mut Frame, area: Rect, _state: &mut AppState, theme: &Theme) {
    let [due_tody, reviewed, streak] = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ])
    .spacing(1)
    .areas(area);

    let card = Block::default()
        .borders(Borders::all())
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(theme.bg));

    for area in [due_tody, reviewed, streak] {
        frame.render_widget(&card, area);
    }
}
