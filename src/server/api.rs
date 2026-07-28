use axum::{Json, extract::State};
use sqlx::SqlitePool;

use crate::server::{actors::DashboardCard, db, error::ServerError};

pub async fn get_dashboard(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<DashboardCard>>, ServerError> {
    let cards = db::fetch_all_dashboard_cards(&pool).await?;

    Ok(Json(cards))
}
