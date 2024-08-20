use crate::{models::db::data_all::AllData, server::server::AppState};
use axum::{extract::State, response::IntoResponse, Json};
use reqwest::StatusCode;
use std::sync::Arc;
use tokio::time::Instant;

//? Получение списка клиентов с данными */
pub async fn all_client(State(data): State<Arc<AppState>>) -> impl IntoResponse {
    let start_time: Instant = Instant::now();

    let resp = match AllData::all_data(data.db.clone()).await {
        Ok(data) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "status": "ok",
                "response_time": format!("{} ms", start_time.elapsed().as_millis()),
                "data": data,
            })),
        ),

        Err(err) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "status": "error",
                "response_time": format!("{} ms", start_time.elapsed().as_millis()),
                "err": err.to_string(),
            })),
        ),
    };

    resp
}
