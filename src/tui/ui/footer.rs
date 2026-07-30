use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::tui::{AppState, theme::Theme};

pub fn render_footer(frame: &mut Frame, area: Rect, _state: &mut AppState, theme: &Theme) {
    let text_style = Style::new().fg(theme.ink).bold();

    let text = vec![Line::from(vec![
        Span::styled("↑/ ↓ move", text_style),
        Span::raw(" "),
        Span::styled("enter review", text_style),
        Span::raw(" "),
        Span::styled("tab details", text_style),
        Span::raw(" "),
        Span::styled("q quit", text_style),
    ])];

    let footer = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::new().fg(theme.ink)),
        )
        .alignment(Alignment::Center);

    frame.render_widget(footer, area);
}
