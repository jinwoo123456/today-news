use axum::{routing::post, Json, Router, extract::State,http::StatusCode};

async fn signup(Json(req): Json<SignupReq>) -> Result<Json<SignupRes>, StatusCode> {
    Ok(Json(SignupRes {
        name: req.name,
        email: req.email,
    }))
}