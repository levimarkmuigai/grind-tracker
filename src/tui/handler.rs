use crate::tui::AppMode;
use ratatui::crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, PartialEq)]
pub enum AppAction {
    Quit,
    SelectUp,
    SelectDown,
    OpenReview,
    SubmitRating(u8),
    CloseReview,
    ReviewSubmitted,
    ShowToast { message: String, kind: ToastKind },
    ClearToast,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastKind {
    Info,
    Success,
    Error,
}

pub fn map_event(key: KeyEvent, mode: &AppMode) -> AppAction {
    match mode {
        AppMode::Dashboard => match key.code {
            KeyCode::Char('q') => AppAction::Quit,
            KeyCode::Up => AppAction::SelectUp,
            KeyCode::Down => AppAction::SelectDown,
            KeyCode::Enter => AppAction::OpenReview,
            _ => AppAction::None,
        },
        AppMode::Review => match key.code {
            KeyCode::Char('1') => AppAction::SubmitRating(1),
            KeyCode::Char('2') => AppAction::SubmitRating(2),
            KeyCode::Char('3') => AppAction::SubmitRating(3),
            KeyCode::Char('4') => AppAction::SubmitRating(4),
            KeyCode::Esc => AppAction::CloseReview,
            _ => AppAction::None,
        },
    }
}
