use sqlx::SqlitePool;

use crate::server::{
    actors::{DashboardCard, FsrsCard, LeetcodeProblem, Level},
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

pub async fn fetch_all_dashboard_cards(
    pool: &SqlitePool,
) -> Result<Vec<DashboardCard>, ServerError> {
    Ok(sqlx::query_as!(
        DashboardCard,
        r#"SELECT
            p.id AS "problem_id!",
            p.title AS "title!",
            p.frontend_id AS "frontend_id!",
            p.topic_tag AS "topic_tag!",
            p.level AS "level!: Level",
            f.stability AS "stability!",
            f.difficulty AS "difficulty!",
            f.due_date AS "due_date!: chrono::NaiveDateTime"
            FROM leetcode_problems p
            JOIN fsrs_cards f ON f.problem_id = p.id
            "#
    )
    .fetch_all(pool)
    .await?)
}
