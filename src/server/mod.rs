use std::env;

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

use crate::server::leetcode_client::seed_db_if_needed;

pub mod actors;
pub mod api;
pub mod db;
pub mod error;
pub mod leetcode_client;

pub async fn run_server() {
    let db_url = env::var("DATABASE_URL").expect("database url not found");
    let addr = env::var("ADDR").expect("connection addr not found");
    let pool = build_pool(&db_url);

    if let Err(err) = seed_db_if_needed(&pool).await {
        tracing::error!("failed to initialize problem set {:?}", err);
    }

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("bind failed");

    tracing::info!("listening on http://{}", addr);

    let router = build_router(pool);
    axum::serve(listener, router)
        .with_graceful_shutdown(signal())
        .await
        .expect("server start error");
}

fn build_pool(db_url: &str) -> SqlitePool {
    SqlitePoolOptions::new()
        .max_connections(8)
        .connect_lazy(db_url)
        .expect("failed to build connection pool")
}

async fn signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install crtl+c handler");
}

fn build_router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/api/dashboard", get(api::get_dashboard))
        .route("/api/stats", get(api::get_stat_cards_data))
        .route("/api/review", post(api::submit_review))
        .with_state(pool)
}
