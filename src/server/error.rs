#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("database: {0}")]
    Database(#[from] sqlx::Error),

    #[error("axum: {0}")]
    Server(#[from] axum::Error),
}
