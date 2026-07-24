use std::env;

use axum::Router;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

pub mod actors;
pub mod db;
pub mod error;

pub async fn run_server() {
    let db_url = env::var("DATABASE_URL").expect("database url not found");
    let addr = env::var("ADDR").expect("connection addr not found");
    let pool = build_pool(&db_url);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("bind failed");

    tracing::info!("listening on http://{}", addr);

    let router = Router::new().with_state(pool);
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
