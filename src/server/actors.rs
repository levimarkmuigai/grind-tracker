use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, PartialEq, Deserialize)]
pub struct TopicTag {
    pub name: String,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, Type)]
#[sqlx(type_name = "Text", rename_all = "PascalCase")]
pub enum Level {
    Easy,
    Medium,
    Hard,
}

impl Level {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Level::Easy => "easy",
            Level::Medium => "medium",
            Level::Hard => "hard",
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ProblemFromLeetcode {
    #[serde(rename = "questionFrontendId")]
    pub frontend_id: String,

    pub title: String,

    #[serde(rename = "difficulty")]
    pub level: Level,

    #[serde(rename = "titleSlug")]
    pub slug: String,

    #[serde(rename = "topicTags")]
    pub topic_tag: Vec<TopicTag>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct LeetcodeProblem {
    pub id: i64,
    pub frontend_id: String,
    pub title: String,
    pub slug: String,
    pub topic_tag: String,
    pub level: Level,
}

#[derive(Debug, PartialEq, sqlx::FromRow)]
pub struct FsrsCardRow {
    pub id: i64,
    pub problem_id: i64,
    pub stability: f64,
    pub difficulty: f64,
    pub due_date: Option<i64>,
    pub state: i64,
    pub reps: i64,
    pub lapses: i64,
    pub last_sync_at: Option<i64>,
}

#[derive(Debug, PartialEq, sqlx::FromRow)]
pub struct FsrsCard {
    pub id: i64,
    pub problem_id: i64,
    pub stability: f64,
    pub difficulty: f64,
    pub due_date: Option<DateTime<Utc>>,
    pub state: i64,
    pub reps: i64,
    pub lapses: i64,
    pub last_sync: Option<DateTime<Utc>>,
}

impl From<FsrsCardRow> for FsrsCard {
    fn from(row: FsrsCardRow) -> Self {
        Self {
            id: row.id,
            problem_id: row.problem_id,
            stability: row.stability,
            difficulty: row.difficulty,
            due_date: row.due_date.and_then(|ts| DateTime::from_timestamp(ts, 0)),
            state: row.state,
            reps: row.reps,
            lapses: row.lapses,
            last_sync: row
                .last_sync_at
                .and_then(|ts| DateTime::from_timestamp(ts, 0)),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, sqlx::FromRow)]
pub struct TableDataRow {
    pub problem_id: i64,
    pub frontend_id: String,
    pub title: String,
    pub diff: Level,
    pub topic: String,
    pub state: i64,
    pub due: Option<i64>,
    pub reps: i64,
    pub lapses: i64,
}

#[derive(Debug, PartialEq, Clone, Serialize, sqlx::FromRow)]
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

impl From<TableDataRow> for TableData {
    fn from(row: TableDataRow) -> Self {
        Self {
            problem_id: row.problem_id,
            frontend_id: row.frontend_id,
            title: row.title,
            diff: row.diff,
            topic: row.topic,
            state: row.state,
            due: row.due.and_then(|ts| DateTime::from_timestamp(ts, 0)),
            reps: row.reps,
            lapses: row.lapses,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Stats {
    pub due_today: i64,
    pub reviewed: i64,
    pub streak: i64,
}
