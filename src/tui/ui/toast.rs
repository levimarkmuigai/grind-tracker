use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

use crate::tui::{handler::ToastKind, theme::Theme, toast::Toast};

pub fn render_toast(frame: &mut Frame, toast: &Toast, theme: &Theme) {
    let area = frame.area();

    let width = 40.min(area.width.saturating_sub(4));
    let height = 3;

    let x = area.width.saturating_sub(width + 2);
    let y = 1;

    let toast_area = Rect {
        x,
        y,
        width,
        height,
    };

    frame.render_widget(Clear, toast_area);

    let (border_color, title) = match toast.kind {
        ToastKind::Info => (theme.blue_ink, "info"),
        ToastKind::Success => (theme.green_ink, "success"),
        ToastKind::Error => (theme.red_ink, "error"),
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .title(title)
        .title_style(
            Style::default()
                .fg(border_color)
                .add_modifier(Modifier::BOLD),
        );

    let paragraph = Paragraph::new(toast.message.as_str())
        .block(block)
        .style(Style::default().fg(theme.ink))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, toast_area);
}
