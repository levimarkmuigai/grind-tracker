use chrono::{NaiveDateTime, Utc};
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Cell, Row, Table},
};

use crate::tui::{AppState, client::Level, theme::Theme};

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

            let due_style = if let Some(date) = x.due {
                due_date_style(date, now, theme)
            } else {
                Style::default()
            };

            Row::new(vec![
                Cell::from(x.frontend_id.clone()),
                Cell::from(truncate(&x.title, 45)),
                Cell::from(x.diff.as_str().to_string()).style(diff_style),
                Cell::from(truncate(&x.topic, 60)),
                Cell::from(state_label).style(state_style),
                if let Some(date) = x.due {
                    Cell::from(format_due_date(date, now)).style(due_style)
                } else {
                    Cell::from("start")
                },
            ])
            .height(2)
        })
        .collect();

    let widths = [
        Constraint::Length(9),
        Constraint::Length(47),
        Constraint::Length(9),
        Constraint::Length(62),
        Constraint::Length(11),
        Constraint::Length(10),
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
    let days_diff = (due.date() - now.date()).num_days();

    match days_diff {
        d if d < 0 => format!("{}", -d),
        0 => "today".into(),
        d => format!("in {}d", d),
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
