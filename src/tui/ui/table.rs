use chrono::{NaiveDateTime, Utc};
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Cell, Row, Table},
};

use crate::{
    server::actors::Level,
    tui::{AppState, theme::Theme},
};

pub fn render_table(frame: &mut Frame, area: Rect, state: &mut AppState, theme: &Theme) {
    let header = ["id", "title", "diff", "topics", "state", "due"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(Style::new().bg(theme.highlight))
        .bold()
        .height(1);

    let now = Utc::now().naive_utc();

    let rows: Vec<Row> = state
        .table_data
        .iter()
        .map(|x| {
            let diff_style = match x.diff {
                Level::Easy => Style::default().fg(theme.green_ink).bold(),
                Level::Medium => Style::default().fg(theme.yellow_ink).bold(),
                Level::Hard => Style::default().fg(theme.red_ink).bold(),
            };

            let (state_label, state_style) = state_style(x.state, theme);

            let due_style = due_date_style(x.due, now, theme);

            Row::new(vec![
                Cell::from(x.frontend_id.clone()),
                Cell::from(truncate(&x.title, 32)),
                Cell::from(x.diff.as_str().to_string()).style(diff_style),
                Cell::from(truncate(&x.topic, 88)),
                Cell::from(state_label).style(state_style),
                Cell::from(format_due_date(x.due, now)).style(due_style),
            ])
        })
        .collect();

    let widths = [
        Constraint::Length(8),
        Constraint::Length(34),
        Constraint::Length(8),
        Constraint::Length(88),
        Constraint::Length(10),
        Constraint::Length(9),
    ];

    let pointer = "▶ ";

    let highlight_symbol: Vec<Line> = vec![pointer.into(), "".into(), "".into(), "".into()];

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::new().fg(theme.bg)),
        )
        .highlight_symbol(Text::from(highlight_symbol).style(Style::default().fg(theme.highlight)))
        .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

    frame.render_stateful_widget(table, area, &mut state.table_state);
}

fn state_style(state: i64, theme: &Theme) -> (&'static str, Style) {
    match state {
        0 => ("new", Style::new().fg(theme.ink).bold()),
        1 => ("learning", Style::new().fg(theme.ink).bold()),
        2 => ("review", Style::new().fg(theme.blue_ink)),
        3 => ("relearn", Style::new().fg(theme.red_ink)),
        _ => ("?", Style::new().fg(theme.ink)),
    }
}

fn format_due_date(due: NaiveDateTime, now: NaiveDateTime) -> String {
    let delta = due.signed_duration_since(now);
    match delta.num_hours() {
        h if h < 0 => format!("{}d", -delta.num_days().max(1)),
        h if h < 24 => "today".into(),
        h => format!("in {}d", h / 24),
    }
}

fn due_date_style(due: NaiveDateTime, now: NaiveDateTime, theme: &Theme) -> Style {
    if due < now {
        Style::new().fg(theme.red_ink).add_modifier(Modifier::BOLD)
    } else {
        Style::new().fg(theme.ink)
    }
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.chars().count() <= max_len {
        s.to_string()
    } else {
        let mut trancated: String = s.chars().take(max_len.saturating_sub(1)).collect();

        trancated.push('…');

        trancated
    }
}
