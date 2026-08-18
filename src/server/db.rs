use anyhow::Result;
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::SqlitePool;

use crate::server::{
    actors::{FsrsCard, LeetcodeProblem, Level, TableData},
    api::{CalculatedData, ReviewPayload},
    error::ServerError,
};

pub async fn insert_problem_with_card(
    pool: &SqlitePool,
    frontend_id: String,
    title: String,
    slug: String,
    topic_tag: String,
    level: Level,
) -> Result<(), ServerError> {
    let mut tx = pool.begin().await?;

    let problem_result = sqlx::query!(
        r#"
        INSERT INTO leetcode_problems (frontend_id, title, slug, topic_tag, level)
        VALUES($1,$2,$3,$4,$5)
        "#,
        frontend_id,
        title,
        slug,
        topic_tag,
        level,
    )
    .execute(&mut *tx)
    .await?;

    let problem_id = problem_result.last_insert_rowid();

    sqlx::query!(
        r#"
        INSERT INTO fsrs_cards (problem_id)
        VALUES($1)
        "#,
        problem_id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}

pub async fn fetch_problem_by_id(
    pool: &SqlitePool,
    id: i64,
) -> Result<LeetcodeProblem, ServerError> {
    Ok(sqlx::query_as!(
        LeetcodeProblem,
        r#"SELECT
            id,
            frontend_id AS "frontend_id!",
            title AS "title!",
            slug AS "slug!",
            topic_tag AS "topic_tag!",
            level AS "level!: Level"
        FROM leetcode_problems WHERE id = $1"#,
        id,
    )
    .fetch_one(pool)
    .await?)
}

pub async fn pareto50_count(pool: &SqlitePool) -> Result<i64, ServerError> {
    let count = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) FROM leetcode_problems
        "#
    )
    .fetch_one(pool)
    .await?;

    Ok(count)
}

pub async fn fetch_card_by_id(pool: &SqlitePool, id: i64) -> Result<FsrsCard, ServerError> {
    Ok(sqlx::query_as!(
        FsrsCard,
        r#"SELECT
            id,
            problem_id,
            stability,
            difficulty,
            due_date AS "due_date: chrono::NaiveDateTime",
            state,
            reps,
            lapses,
            last_sync_at AS "last_sync!: chrono::NaiveDateTime"
            FROM fsrs_cards
            WHERE id = $1"#,
        id
    )
    .fetch_one(pool)
    .await?)
}

pub async fn fetch_calculation_data_by_problem_id(
    pool: &SqlitePool,
    problem_id: i64,
) -> Result<(f32, f32, Option<NaiveDateTime>), ServerError> {
    let row = sqlx::query!(
        r#"
    SELECT
    stability AS "stability: f32",
    difficulty AS "difficulty: f32",
    last_sync_at AS "last_sync_at: chrono::NaiveDateTime"
    FROM fsrs_cards
    WHERE problem_id = $1
        "#,
        problem_id
    )
    .fetch_one(pool)
    .await?;

    Ok((row.stability, row.difficulty, row.last_sync_at))
}

pub async fn fetch_all_problems(pool: &SqlitePool) -> Result<Vec<LeetcodeProblem>, ServerError> {
    let problems = sqlx::query_as!(
        LeetcodeProblem,
        r#"SELECT 
            id, 
            frontend_id AS "frontend_id!", 
            title AS "title!", 
            slug AS "slug!", 
            topic_tag AS "topic_tag!", 
            level AS "level!: Level" 
        FROM leetcode_problems"#
    )
    .fetch_all(pool)
    .await?;

    Ok(problems)
}

pub async fn fetch_all_cards(pool: &SqlitePool) -> Result<Vec<FsrsCard>, ServerError> {
    let cards = sqlx::query_as!(
        FsrsCard,
        r#"SELECT
            id,
            problem_id,
            stability,
            difficulty,
            due_date AS "due_date: chrono::NaiveDateTime",
            state,
            reps,
            lapses,
            last_sync_at AS "last_sync!: chrono::NaiveDateTime"
            FROM fsrs_cards"#
    )
    .fetch_all(pool)
    .await?;

    Ok(cards)
}

pub async fn fetch_table_data(pool: &SqlitePool) -> Result<Vec<TableData>, ServerError> {
    Ok(sqlx::query_as!(
        TableData,
        r#"SELECT
        p.id AS "problem_id",
            p.frontend_id AS "frontend_id!",
            p.title AS "title",
            p.topic_tag AS "topic!",
            p.level AS "diff!: Level",
            f.state AS "state!",
            f.reps AS "reps",
            f.lapses AS "lapses",
            f.due_date AS "due!: chrono::NaiveDateTime"
            FROM leetcode_problems p
            JOIN fsrs_cards f ON f.problem_id = p.id
            "#
    )
    .fetch_all(pool)
    .await?)
}

pub async fn fetch_due_today_reviewed(pool: &SqlitePool) -> Result<(i64, i64), ServerError> {
    let now = chrono::Local::now().naive_local();
    let start_day = now.date().and_hms_opt(0, 0, 0).unwrap();
    let end_day = start_day + chrono::Duration::days(1);
    let in_24h = now + chrono::Duration::hours(24);

    let (due_today, reviewed): (i64, i64) = sqlx::query_as(
        r#"
        SELECT
        (SELECT COUNT(*) FROM fsrs_cards WHERE due_date <= ?),
        (SELECT COUNT(*) FROM review_logs WHERE reviewed_at >= ? AND reviewed_at < ?)
        "#,
    )
    .bind(in_24h)
    .bind(start_day)
    .bind(end_day)
    .fetch_one(pool)
    .await?;

    Ok((due_today, reviewed))
}

pub async fn fetch_streak_dates(pool: &SqlitePool) -> Result<Vec<NaiveDate>, ServerError> {
    let dates: Vec<NaiveDate> = sqlx::query_scalar::<_, NaiveDate>(
        r#"
    SELECT DISTINCT DATE(reviewed_at) FROM review_logs  ORDER BY DATE(reviewed_at) DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(dates)
}

pub async fn submit_review_update_fsrs(
    pool: &SqlitePool,
    review_data: ReviewPayload,
    fsrs_data: CalculatedData,
) -> Result<(), ServerError> {
    let mut tx = pool.begin().await?;

    sqlx::query!(
        r#"
    UPDATE fsrs_cards
    SET
    stability = ?,
    difficulty = ?,
    due_date = ?
    WHERE problem_id = ?
        "#,
        fsrs_data.stability,
        fsrs_data.difficulty,
        fsrs_data.due_date,
        review_data.problem_id
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query!(
        r#"
        INSERT INTO review_logs (problem_id, rating) VALUES(?,?)
        "#,
        review_data.problem_id,
        review_data.review
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(())
}
