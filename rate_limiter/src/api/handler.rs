use std::sync::{Arc, Mutex};
use axum::{extract::State, http::StatusCode};
use crate::limiter::token_bucket::TokenBucket;

pub type SharedLimiter = Arc<Mutex<TokenBucket>>;

pub async fn handle_request(
    State(limiter): State<SharedLimiter>
) -> StatusCode {
    let mut limiter = limiter.lock().unwrap();

    if limiter.allow_request() {
        StatusCode::OK
    } else {
        StatusCode::TOO_MANY_REQUESTS
    }
}