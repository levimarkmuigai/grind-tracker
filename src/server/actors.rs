use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, PartialEq, Deserialize)]
pub struct TopicTag {
    pub name: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Type)]
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

#[derive(Debug, PartialEq)]
pub struct FsrsCard {
    pub id: i64,
    pub problem_id: i64,
    pub stability: f64,
    pub difficulty: f64,
    pub due_date: NaiveDateTime,
    pub state: i64,
    pub reps: i64,
    pub lapses: i64,
    pub last_sync: NaiveDateTime,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct TableData {
    pub frontend_id: String,
    pub title: String,
    pub diff: Level,
    pub topic: String,
    pub state: i64,
    pub due: NaiveDateTime,
}

impl TableData {
    pub fn as_array(&self) -> Vec<String> {
        vec![
            self.frontend_id.clone(),
            self.title.clone(),
            self.diff.as_str().to_string(),
            self.topic.clone(),
            self.state.to_string(),
            self.due.to_string(),
        ]
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Stats {
    pub due_today: i64,
    pub reviewed: i64,
    pub streak: i64,
}
