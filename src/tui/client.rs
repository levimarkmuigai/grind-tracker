use std::env;

use chrono::{DateTime, Utc};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

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
    pub due: Option<DateTime<Utc>>,
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
    dashboard_url: String,
    stats_url: String,
    review_url: String,
}

impl Api {
    pub fn new() -> Result<Self, reqwest::Error> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;
        Ok(Self {
            client,
            dashboard_url: env::var("DASH_URL").expect("dashboard endpoint not set"),
            stats_url: env::var("STATS_URL").expect("stats endpoint not set"),
            review_url: env::var("REVIEW_URL").expect("review endpoint not set"),
        })
    }

    pub fn seed_dash_data(&self) -> Result<(Vec<TableData>, Stats), reqwest::Error> {
        let table_data: Vec<TableData> = self
            .client
            .get(&self.dashboard_url)
            .send()?
            .error_for_status()?
            .json()?;

        let stats: Stats = self
            .client
            .get(&self.stats_url)
            .send()?
            .error_for_status()?
            .json()?;

        Ok((table_data, stats))
    }

    pub fn submit_review(&self, problem_id: i64, review: u8) -> Result<(), reqwest::Error> {
        let payload = ReviewPayload { problem_id, review };

        self.client
            .post(&self.review_url)
            .json(&payload)
            .send()?
            .error_for_status()?;

        Ok(())
    }
}
