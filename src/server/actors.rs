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
pub struct DashboardCard {
    pub problem_id: i64,
    pub title: String,
    pub frontend_id: String,
    pub topic_tag: String,
    pub level: Level,
    pub stability: f64,
    pub difficulty: f64,
    pub due_date: NaiveDateTime,
}
