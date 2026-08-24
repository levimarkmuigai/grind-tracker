use chrono::NaiveDateTime;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "http://localhost:3000";

#[derive(Debug, PartialEq, Clone, Deserialize)]
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
    pub due: Option<NaiveDateTime>,
    pub reps: i64,
    pub lapses: i64,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Stats {
    pub due_today: i64,
    pub reviewed: i64,
    pub streak: i64,
}

#[derive(Serialize)]
pub struct ReviewPayload {
    problem_id: i64,
    review: u8,
}

#[derive(Clone)]
pub struct Api {
    client: Client,
    base: String,
}

impl Api {
    pub fn new() -> Result<Self, reqwest::Error> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;
        Ok(Self {
            client,
            base: BASE_URL.to_string(),
        })
    }

    pub fn seed_dash_data(&self) -> Result<(Vec<TableData>, Stats), reqwest::Error> {
        let table_data: Vec<TableData> = self
            .client
            .get(format!("{}/api/dashboard", self.base))
            .send()?
            .error_for_status()?
            .json()?;

        let stats: Stats = self
            .client
            .get(format!("{}/api/stats", self.base))
            .send()?
            .error_for_status()?
            .json()?;

        Ok((table_data, stats))
    }

    pub fn submit_review(&self, problem_id: i64, review: u8) -> Result<(), reqwest::Error> {
        let payload = ReviewPayload { problem_id, review };

        self.client
            .post(format!("{}/api/review", self.base))
            .json(&payload)
            .send()?
            .error_for_status()?;

        Ok(())
    }
}
