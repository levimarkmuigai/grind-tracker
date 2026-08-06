use anyhow::Result;
use axum::{Json, extract::State};
use chrono::{Duration, Local, NaiveDate};
use sqlx::SqlitePool;

use crate::server::{
    actors::{Stats, TableData},
    db,
    error::ServerError,
};

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

    let today = Local::now().naive_local().date();

    let streak = calculate_streak(dates, today);

    Ok(Json(Stats {
        due_today,
        reviewed,
        streak,
    }))
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
        } else {
            break;
        }
    }

    streak
}
