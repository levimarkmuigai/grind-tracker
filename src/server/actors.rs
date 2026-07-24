use chrono::NaiveDateTime;

#[derive(Debug, PartialEq)]
pub struct LeetcodeProblem {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub category: String,
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
