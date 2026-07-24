use sqlx::SqlitePool;

use crate::server::{
    actors::{FsrsCard, LeetcodeProblem},
    error::ServerError,
};

pub async fn insert_problem_with_card(
    pool: &SqlitePool,
    title: String,
    slug: String,
    category: String,
) -> Result<(), ServerError> {
    let mut tx = pool.begin().await?;

    let problem_result = sqlx::query!(
        r#"
    INSERT INTO leetcode_problems (title, slug, category)
    VALUES($1,$2,$3)"#,
        title,
        slug,
        category
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
        "SELECT * FROM leetcode_problems WHERE id = $1",
        id,
    )
    .fetch_one(pool)
    .await?)
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
    let problems: Vec<LeetcodeProblem> =
        sqlx::query_as!(LeetcodeProblem, r#"SELECT * FROM leetcode_problems"#)
            .fetch_all(pool)
            .await?;

    Ok(problems)
}

pub async fn fetch_all_cards(pool: &SqlitePool) -> Result<Vec<FsrsCard>, ServerError> {
    let cards: Vec<FsrsCard> = sqlx::query_as!(
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
