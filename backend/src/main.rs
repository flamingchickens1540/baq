mod model;

use crate::model::QueueState;
#[allow(unused_imports)]
use axum::{
    Json, Router, debug_handler,
    extract::{Query, State},
    routing::{get, post},
};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

static HOST: &str = "127.0.0.1:3000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let state = QueueState::default();

    let app: Router<()> = Router::new()
        .route("/health", get(health))
        .route("/queue", get(get_queue))
        .route("/queue", post(queue_robot))
        .route("/match", post(new_match))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(HOST).await.unwrap();
    info!("Server running at: {HOST}");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn health() -> &'static str {
    "Hello World!"
}

async fn get_queue(State(state): State<QueueState>) -> Json<Vec<String>> {
    Json::from(
        state
            .get_queue()
            .into_iter()
            .cloned()
            .collect::<Vec<String>>(),
    )
}
async fn queue_robot(
    State(mut state): State<QueueState>,
    Query(team): Query<String>,
) -> Json<Vec<String>> {
    Json::from(state.queue_robot(team).clone())
}
async fn new_match() {}
