use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};

#[derive(Debug, PartialEq)]
pub enum AppAction {
    Quit,
    None,
}

pub fn map_event(event: Event) -> AppAction {
    match event {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') | KeyCode::Esc => AppAction::Quit,
            _ => AppAction::None,
        },
        _ => AppAction::None,
    }
}

#[cfg(test)]
mod test {
    use ratatui::crossterm::event::{KeyEvent, KeyEventState, KeyModifiers};

    use super::*;

    #[test]
    fn test_quit_action() {
        let q_press = Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        });

        let esc_press = Event::Key(KeyEvent {
            code: KeyCode::Esc,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        });

        assert_eq!(map_event(q_press), AppAction::Quit);
        assert_eq!(map_event(esc_press), AppAction::Quit);
    }
}
