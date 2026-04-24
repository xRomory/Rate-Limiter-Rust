use std::sync::{Arc, Mutex};

use axum::{routing::get, Router};
use rate_limiter::{
    api::handler::handle_request,
    token_bucket::TokenBucket
};

#[tokio::main]
async fn main() {
    let limiter = Arc::new(Mutex::new(TokenBucket::new(5, 2.0)));

    let app = Router::new()
        .route("/request", get(handle_request))
        .with_state(limiter);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server is running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
