use axum::{Json, extract::State};
use sqlx::SqlitePool;

use crate::server::{actors::TableData, db, error::ServerError};

pub async fn get_dashboard(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<TableData>>, ServerError> {
    let cards = db::fetch_table_data(&pool).await?;

    Ok(Json(cards))
}
