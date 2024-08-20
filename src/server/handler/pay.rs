use crate::{models::db::pay::PayTable, server::server::AppState};
use axum::{extract::State, response::IntoResponse, Json};
use reqwest::StatusCode;
use std::sync::Arc;
use tokio::time::Instant;

//? Получение всех оплат */
pub async fn list_pay(State(data): State<Arc<AppState>>) -> impl IntoResponse {
    let start_time: Instant = Instant::now();

    let resp = match PayTable::get_all_pay(data.db.clone()).await {
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

//? Добавление оплаты */
pub async fn add_pay_client(
    State(data): State<Arc<AppState>>,
    Json(body): Json<PayTable>,
) -> impl IntoResponse {
    let start_time: Instant = Instant::now();

    let id: bool = body.id.is_some();

    if id == false {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "status": "error",
                "response_time": format!("{} ms", start_time.elapsed().as_millis()),
                "error": "Не указан ID",
            })),
        );
    }

    let resp = match PayTable::add_new_pay(&body, data.db.clone()).await {
        Ok(data) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "status": "ok",
                "response_time": format!("{} ms", start_time.elapsed().as_millis()),
                "data": data,
            })),
        ),

        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "status": "error",
                "response_time": format!("{} ms", start_time.elapsed().as_millis()),
                "error": err.to_string(),
            })),
        ),
    };

    resp
}

//? Обнавление суммы лота */
pub async fn update_pay(
    State(data): State<Arc<AppState>>,
    Json(body): Json<PayTable>,
) -> impl IntoResponse {
    let start_time: Instant = Instant::now();

    let id: bool = body.id.is_some();

    if id == false {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "status": "error",
                "response_time": format!("{} ms", start_time.elapsed().as_millis()),
                "error": "Не указан ID",
            })),
        );
    }

    let resp = match PayTable::update_pay(&body, data.db.clone()).await {
        Ok(data) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "status": "ok",
                "response_time": format!("{} ms", start_time.elapsed().as_millis()),
                "data": data,
            })),
        ),

        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "status": "error",
                "response_time": format!("{} ms", start_time.elapsed().as_millis()),
                "error": err.to_string(),
            })),
        ),
    };

    resp
}
