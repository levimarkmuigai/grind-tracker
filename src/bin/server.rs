use grind_tracker::server::run_server;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();

    run_server().await;
}
