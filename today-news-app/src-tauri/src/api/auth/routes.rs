use axum::{
    routing::{get, post},
    Router,
};
use super::handlers;




pub fn router() -> Router {
    Router::new()
        .route("/signup", post(handlers::signup))
        // .route("/login", post(handlers::login))
}
