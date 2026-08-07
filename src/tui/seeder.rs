use chrono::NaiveDateTime;
use reqwest::blocking::Client;
use serde::Deserialize;
use sqlx::Type;

const BASE_URL: &str = "http://localhost:3000";

#[derive(Debug, PartialEq, Clone, Deserialize, Type)]
#[sqlx(type_name = "Text", rename_all = "PascalCase")]
pub enum Level {
    Easy,
    Medium,
    Hard,
}

impl Level {
    pub fn as_str(&self) -> &str {
        match self {
            Level::Easy => "easy",
            Level::Medium => "medium",
            Level::Hard => "hard",
        }
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct TableData {
    pub problem_id: i64,
    pub frontend_id: String,
    pub title: String,
    pub diff: Level,
    pub topic: String,
    pub state: i64,
    pub due: NaiveDateTime,
    pub reps: i64,
    pub lapses: i64,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Stats {
    pub due_today: i64,
    pub reviewed: i64,
    pub streak: i64,
}

pub fn seed_dash_data() -> Result<(Vec<TableData>, Stats), reqwest::Error> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let table_data: Vec<TableData> = client
        .get(format!("{}/api/dashboard", BASE_URL))
        .send()?
        .error_for_status()?
        .json()?;

    let stats: Stats = client
        .get(format!("{}/api/stats", BASE_URL))
        .send()?
        .error_for_status()?
        .json()?;

    Ok((table_data, stats))
}
