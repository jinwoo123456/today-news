use axum::{routing::post, Json, Router, extract::State,http::StatusCode};

use super::dto;

pub async fn signup(Json(req): Json<dto::SignupReq>) -> Result<Json<dto::SignupRes>, StatusCode> {
    Ok(Json(dto::SignupRes {
        name: req.name,
        email: req.email,
        password: req.password,
    }))
}