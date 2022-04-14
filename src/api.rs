use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use crate::Image;

async fn start_server() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/upload", post(upload));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn upload(Json(payload): Json<Image>) -> impl IntoResponse{

}   