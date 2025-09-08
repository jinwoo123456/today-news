#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api; // 없다면 주석 처리
mod db;  // 없다면 주석 처리

use tauri::Manager;
use axum::{Router, routing::get};
use tower_http::cors::CorsLayer;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
fn build_axum() -> Router {
    Router::new()
        .merge(crate::api::auth::routes::router()) 
        .layer(CorsLayer::permissive())
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()            
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let handle = app.handle().clone();

            // Axum 서버 비동기 실행
            tauri::async_runtime::spawn(async move {
                let app = build_axum();

                let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
                    .await
                    .expect("failed to bind 127.0.0.1:3000");

                if let Err(e) = axum::serve(listener, app).await {
                    eprintln!("axum serve error: {e}");
                }
            });

            Ok(()) 
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

