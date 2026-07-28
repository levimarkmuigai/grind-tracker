use axum::response::IntoResponse;
use reqwest::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("database: {0}")]
    Database(#[from] sqlx::Error),

    #[error("axum: {0}")]
    Server(#[from] axum::Error),

    #[error("reqwest: {0}")]
    Network(#[from] reqwest::Error),

    #[error("serde_json: {0}")]
    Parse(#[from] serde_json::Error),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("database/server error: {:?}", self);

        (StatusCode::INTERNAL_SERVER_ERROR, "internal server error").into_response()
    }
}
