use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::tui::{
    seeder::{Level, TableData},
    theme::Theme,
};

pub fn render_review(frame: &mut Frame, area: Rect, problem: &TableData, theme: &Theme) {
    let [_, center, _] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(16),
        Constraint::Fill(1),
    ])
    .areas(area);

    let [_, panel, _] = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Percentage(52),
        Constraint::Fill(1),
    ])
    .areas(center);

    frame.render_widget(Clear, panel);

    let outer = Block::bordered()
        .title(Line::from(vec![
            Span::raw(format!(" #{} ", problem.frontend_id)),
            Span::styled(
                problem.title.clone(),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
        ]))
        .border_style(theme.ink);

    let inner = outer.inner(panel);

    frame.render_widget(outer, panel);

    let [diff_row, stats_row, rating_row, footer_row] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(8),
        Constraint::Length(3),
    ])
    .areas(inner);

    render_diff_topics(frame, diff_row, problem, theme);
    render_stats(frame, stats_row, problem, theme);
    render_ratings(frame, rating_row, theme);
    render_footer(frame, footer_row, theme);
}

fn render_diff_topics(frame: &mut Frame, area: Rect, problem: &TableData, theme: &Theme) {
    let style = match problem.diff {
        Level::Easy => Style::new().fg(theme.green_ink),
        Level::Medium => Style::new().fg(theme.yellow_ink),
        Level::Hard => Style::new().fg(theme.red_ink),
    };

    let text = vec![
        Line::from(Span::styled(problem.diff.as_str(), style)),
        Line::from(Span::styled(problem.topic.clone(), theme.ink)),
    ];

    frame.render_widget(Paragraph::new(text), area);
}

fn render_stats(frame: &mut Frame, area: Rect, problem: &TableData, theme: &Theme) {
    let lapses_style = if problem.lapses > 3 {
        theme.red_ink
    } else {
        theme.ink
    };

    let line = Line::from(vec![
        Span::styled("reviews ", theme.ink),
        Span::styled(problem.reps.to_string(), theme.blue_ink),
        Span::raw(" "),
        Span::styled("lapses ", theme.ink),
        Span::styled(problem.lapses.to_string(), lapses_style),
    ]);

    frame.render_widget(
        Paragraph::new(line).block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(theme.ink),
        ),
        area,
    );
}

fn render_ratings(frame: &mut Frame, area: Rect, theme: &Theme) {
    let [r1, r2, r3, r4] = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ])
    .areas(area);

    let ratings = [
        (r1, "1", "again", "blank", theme.red_ink),
        (r2, "2", "hard", "struggled", theme.yellow_ink),
        (r3, "3", "good", "correct", theme.blue_ink),
        (r4, "4", "easy", "trivial", theme.green_ink),
    ];

    for (area, key, label, desc, color) in ratings {
        let text = vec![
            Line::from(Span::styled(
                format!("[{key}]"),
                Style::new().fg(color).bold(),
            )),
            Line::from(label),
            Line::from(Span::styled(desc, theme.ink)),
        ];

        frame.render_widget(
            Paragraph::new(text)
                .block(Block::bordered().border_style(theme.ink))
                .alignment(Alignment::Center),
            area,
        );
    }
}

fn render_footer(frame: &mut Frame, area: Rect, theme: &Theme) {
    let style = Style::new().fg(theme.ink).bold();

    let text = vec![Line::from(vec![
        Span::styled("rate: 1-4", style),
        Span::raw(" | "),
        Span::styled("esc to go back", style),
    ])];

    let footer = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::new().fg(theme.highlight)),
        )
        .alignment(Alignment::Center);

    frame.render_widget(footer, area);
}
