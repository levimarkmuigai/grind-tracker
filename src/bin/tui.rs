use chrono::NaiveDate;
use grind_tracker::{
    server::actors::{Level, Stats, TableData},
    tui::{self, terminal},
};

fn main() -> color_eyre::Result<()> {
    let table_data: Vec<TableData> = mock_table_date();
    let cards_data: Stats = Stats {
        due_today: 3,
        reviewed: 10,
        streak: 5,
    };

    let state = tui::AppState::new(table_data, cards_data);

    let mut terminal = terminal::init()?;

    let theme = tui::theme::Theme::default();

    tui::run(state, &mut terminal, &theme)?;

    ratatui::restore();

    Ok(())
}

fn mock_table_date() -> Vec<TableData> {
    vec![
        TableData {
            frontend_id: "1".to_string(),
            title: "Two Sum".to_string(),
            diff: Level::Easy,
            topic: "Array, Hash Table".to_string(),
            state: 0, // New
            due: NaiveDate::from_ymd_opt(2026, 8, 6)
                .unwrap()
                .and_hms_opt(20, 0, 0)
                .unwrap(),
        },
        TableData {
            frontend_id: "20".to_string(),
            title: "Valid Parentheses".to_string(),
            diff: Level::Easy,
            topic: "String, Stack".to_string(),
            state: 1, // Learning
            due: NaiveDate::from_ymd_opt(2026, 8, 7)
                .unwrap()
                .and_hms_opt(9, 30, 0)
                .unwrap(),
        },
        TableData {
            frontend_id: "56".to_string(),
            title: "Merge Intervals".to_string(),
            diff: Level::Medium,
            topic: "Array, Sorting".to_string(),
            state: 2, // Review
            due: NaiveDate::from_ymd_opt(2026, 8, 8)
                .unwrap()
                .and_hms_opt(14, 0, 0)
                .unwrap(),
        },
        TableData {
            frontend_id: "139".to_string(),
            title: "Word Break".to_string(),
            diff: Level::Medium,
            topic: "Array, Hash Table, Dynamic Programming, Trie".to_string(),
            state: 3, // Relearning
            due: NaiveDate::from_ymd_opt(2026, 8, 6)
                .unwrap()
                .and_hms_opt(21, 15, 0)
                .unwrap(),
        },
        TableData {
            frontend_id: "76".to_string(),
            title: "Minimum Window Substring".to_string(),
            diff: Level::Hard,
            topic: "Hash Table, String, Sliding Window".to_string(),
            state: 2, // Review
            due: NaiveDate::from_ymd_opt(2026, 8, 3)
                .unwrap()
                .and_hms_opt(10, 0, 0)
                .unwrap(),
        },
    ]
}
