mod api;
mod utils;

use axum::{
    http::{header::CONTENT_TYPE, HeaderValue},
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/v1", get(|| async { "Hello, world!" }))
        .route("/api/v1/run", post(api::run::post_run))
        .route("/api/v1/grade", post(api::grade::post_grade))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_headers([CONTENT_TYPE]),
        );

    let listner = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    axum::serve(listner, app).await.unwrap();
}
