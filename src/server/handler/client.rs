use crate::{
    models::{
        db::{
            get_client_list::ClientTableList, new_client::AddNewClientTable,
            update_client::UpdateClientTable,
        },
        server::request_type::Id,
    },
    server::server::AppState,
};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use reqwest::StatusCode;
use std::sync::Arc;
use tokio::time::Instant;
use validator::Validate;

//? Добавление нового клиента */
pub async fn new_client(
    State(data): State<Arc<AppState>>,
    Json(body): Json<AddNewClientTable>,
) -> impl IntoResponse {
    let start_time: Instant = Instant::now();

    let valid_client = AddNewClientTable { ..body.clone() }.validate();

    match valid_client {
        Ok(_) => {}
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "status": "error",
                    "response_time": format!("{} ms", start_time.elapsed().as_millis()),
                    "err": err.to_string(),
                })),
            );
        }
    }

    let resp = match AddNewClientTable::add_to_db(&body, data.db.clone()).await {
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

//? Получение списка клиентов с данными */
pub async fn handler_client_list(State(data): State<Arc<AppState>>) -> impl IntoResponse {
    let start_time: Instant = Instant::now();

    let resp = match ClientTableList::all_clients(data.db.clone()).await {
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

//? Получение одного клиента */
pub async fn handler_client_id(
    State(data): State<Arc<AppState>>,
    param: Option<Query<Id>>,
) -> impl IntoResponse {
    let start_time: Instant = Instant::now();

    let id: bool = param.as_deref().is_some();

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

    let id: i64 = param.unwrap().id;

    let resp = match ClientTableList::get_client_id(id, data.db.clone()).await {
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

//? Обновление данных клиента */
pub async fn update_client(
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateClientTable>,
) -> impl IntoResponse {
    let start_time: Instant = Instant::now();

    let valid_client = UpdateClientTable { ..body.clone() }.validate();

    match valid_client {
        Ok(_) => {}
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "status": "error",
                    "response_time": format!("{} ms", start_time.elapsed().as_millis()),
                    "err": err.to_string(),
                })),
            );
        }
    }

    let resp = match UpdateClientTable::update_client(&body, data.db.clone()).await {
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
