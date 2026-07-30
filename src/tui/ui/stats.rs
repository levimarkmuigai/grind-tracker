use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders},
};

use crate::tui::{AppState, theme::Theme};

pub fn render_stats(frame: &mut Frame, area: Rect, _state: &mut AppState, theme: &Theme) {
    let [due_tody, reviewed, streak] = Layout::horizontal([
        Constraint::Min(40),
        Constraint::Min(40),
        Constraint::Min(40),
    ])
    .areas(area);

    let card = Block::default()
        .borders(Borders::all())
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(theme.ink))
        .style(Style::new().bg(theme.highlight));

    frame.render_widget(card.clone(), reviewed);
    frame.render_widget(card.clone(), due_tody);
    frame.render_widget(card.clone(), streak);
}
