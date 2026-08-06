use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::tui::{AppState, theme::Theme};

pub fn render_stats(frame: &mut Frame, area: Rect, state: &mut AppState, theme: &Theme) {
    let [due_tody, reviewed, streak] = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ])
    .spacing(1)
    .areas(area);

    let card = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(theme.bg));

    let stats = state.cards_data.clone();

    let due_date_text = vec![
        Line::from(Span::styled("due today", Style::new().fg(theme.ink))),
        Line::from(Span::styled(stats.reviewed.to_string(), Style::new().fg(theme.ink)).bold()),
    ];

    let reviewed_text = vec![
        Line::from(Span::styled("reviewed", Style::new().fg(theme.ink))),
        Line::from(Span::styled(stats.reviewed.to_string(), Style::new().fg(theme.ink)).bold()),
    ];

    let streak_string = if stats.streak == 1 {
        format!("{} day", stats.reviewed)
    } else {
        format!("{} days", stats.reviewed)
    };

    let streak_text = vec![
        Line::from(Span::styled("streak", Style::new().fg(theme.ink))),
        Line::from(Span::styled(streak_string, Style::new().fg(theme.ink)).bold()),
    ];

    let cards = [
        (due_tody, due_date_text),
        (reviewed, reviewed_text),
        (streak, streak_text),
    ];

    for (area, text) in cards {
        frame.render_widget(&card, area);

        let inner = card.inner(area);

        let paragraph = Paragraph::new(text).alignment(Alignment::Center);

        let text_height = 2;

        let centerted_y = inner.y + (inner.height.saturating_sub(text_height)) / 2;

        let text_area = Rect {
            x: inner.x,
            y: centerted_y,
            width: inner.width,
            height: text_height.min(inner.height),
        };

        frame.render_widget(paragraph, text_area);
    }
}
