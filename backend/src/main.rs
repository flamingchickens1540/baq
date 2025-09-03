mod model;

use std::sync::Arc;

use crate::model::QueueState;
use axum::http::StatusCode;
#[allow(unused_imports)]
use axum::{
    Json, Router, debug_handler,
    extract::{Query, State},
    routing::{get, post},
};
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

type AppState = Arc<Mutex<QueueState>>;
static HOST: &str = "127.0.0.1:3000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let state = Arc::new(Mutex::new(QueueState::default()));

    let app: Router<()> = Router::new()
        .route("/health", get(health))
        .route("/queue", get(get_queue))
        .route("/queue", post(queue_robot))
        .route("/dequeue", post(dequeue_robot))
        .route("/match", post(new_match))
        .layer(CorsLayer::permissive())
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

async fn get_queue<'a>(State(state): State<AppState>) -> Json<Vec<String>> {
    Json::from(
        state
            .lock()
            .await
            .get_queue()
            .into_iter()
            .collect::<Vec<String>>(),
    )
}

async fn queue_robot<'a>(
    State(state): State<AppState>,
    Json(team): Json<String>,
) -> Json<Vec<String>> {
    let mut state = state.lock().await;
    state.insert_robot(&team);
    Json::from(state.get_queue())
}

async fn dequeue_robot<'a>(
    State(state): State<AppState>,
    Json(team): Json<String>,
) -> (StatusCode, Json<Vec<String>>) {
    let mut state = state.lock().await;
    let status = match state.dequeue_robot(&team) {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::UNPROCESSABLE_ENTITY,
    };
    let queue = Json::from(state.get_queue().clone());

    (status, queue)
}

async fn new_match(State(state): State<AppState>) {
    state.lock().await.pop_6();
}
