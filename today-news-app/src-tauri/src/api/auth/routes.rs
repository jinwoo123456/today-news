use axum::{
    routing::{get, post},
    Router,
};
use super::handlers;




pub fn routes() -> Router<()> {
    Router::<()>::new()
        .without_v07_checks()
        .route("/signup", post(handlers::signup))
        .route("/login", post(handlers::login))
}
