use anyhow::Result;
use axum::{Json, extract::State, response::IntoResponse};
use chrono::{DateTime, Duration, NaiveDate, Utc};
use fsrs::{FSRS, MemoryState};
use reqwest::StatusCode;
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::server::{
    actors::{Stats, TableData},
    db,
    error::ServerError,
};

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct ReviewPayload {
    pub problem_id: i64,
    pub review: u8,
}

#[derive(Debug)]
pub struct CalculatedData {
    pub stability: f32,
    pub difficulty: f32,
    pub due_date: DateTime<Utc>,
}

pub async fn get_dashboard(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<TableData>>, ServerError> {
    let cards = db::fetch_table_data(&pool).await?;

    Ok(Json(cards))
}

pub async fn get_stat_cards_data(
    State(pool): State<SqlitePool>,
) -> Result<Json<Stats>, ServerError> {
    let (due_today_reviewed, dates) = tokio::join!(
        db::fetch_due_today_reviewed(&pool),
        db::fetch_streak_dates(&pool)
    );

    let (due_today, reviewed) = due_today_reviewed?;

    let dates = dates?;

    let today = Utc::now().date_naive();

    let streak = calculate_streak(dates.clone(), today);

    Ok(Json(Stats {
        due_today,
        reviewed,
        streak,
    }))
}

pub async fn submit_review(
    State(pool): State<SqlitePool>,
    Json(payload): Json<ReviewPayload>,
) -> Result<impl IntoResponse, ServerError> {
    tracing::info!(
        "problem-id={}, review={}",
        payload.problem_id,
        payload.review
    );

    let calculated_fsrs = calculate_fsrs(&payload, &pool).await?;

    db::submit_review_update_fsrs(&pool, payload, calculated_fsrs).await?;

    Ok(StatusCode::OK)
}

fn calculate_streak(dates: Vec<NaiveDate>, today: NaiveDate) -> i64 {
    let start_from = if dates.first() == Some(&today) {
        today
    } else {
        today - Duration::days(1)
    };

    let mut expected = start_from;
    let mut streak: i64 = 0;

    for date in dates {
        if date == expected {
            streak += 1;
            expected -= Duration::days(1);
        } else if date < expected {
            break;
        }
    }

    streak
}

async fn calculate_fsrs(
    payload: &ReviewPayload,
    pool: &SqlitePool,
) -> Result<CalculatedData, ServerError> {
    const DESIRED_RETENTION: f32 = 0.9;
    let fsrs = FSRS::default();

    let (stability, difficulty, last_sync_at) =
        db::fetch_calculation_data_by_problem_id(pool, payload.problem_id).await?;

    let previous = if stability > 0.0 {
        Some(MemoryState {
            stability,
            difficulty,
        })
    } else {
        None
    };

    let days_elapsed = match last_sync_at {
        Some(last) if stability > 0.0 => (Utc::now() - last).num_days().max(0) as u32,
        _ => 0,
    };

    let next_states = fsrs.next_states(previous, DESIRED_RETENTION, days_elapsed)?;

    let chosen = match payload.review {
        1 => next_states.again,
        2 => next_states.hard,
        3 => next_states.good,
        4 => next_states.easy,
        _ => next_states.good,
    };

    let interval = chosen.interval.round().max(1.0) as u32;
    let due_date = Utc::now() + chrono::Duration::days(interval as i64);

    Ok(CalculatedData {
        stability: chosen.memory.stability,
        difficulty: chosen.memory.difficulty,
        due_date,
    })
}
