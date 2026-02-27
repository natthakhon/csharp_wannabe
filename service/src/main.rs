mod controllers;
mod app_state;
use axum::{routing::get, Router,routing::post};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let db_url = "postgres://postgres:postgres@localhost:5432/rust";
    let state = std::sync::Arc::new(app_state::AppState{
        repo:pgsql_provider::PgSqlRandomRepo::new(db_url).await.unwrap(),
    });
    let app = Router::new().route("/", get(hello_world))
        .route("/test", post(controllers::test_controller))
        .with_state(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
async fn hello_world() -> String {
    "Hello from the Rust Backend!".to_string()
}