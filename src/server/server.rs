use crate::{
    server::handler::{
        client::{handler_client_id, handler_client_list, new_client, update_client},
        data::all_client,
        pay::{add_pay_client, list_pay, update_pay},
    },
    utils::create_table::create_table,
};
use axum::{
    http::HeaderValue,
    routing::{get, post, put},
    Router,
};
use dotenv::dotenv;
use reqwest::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Method,
};
use std::sync::Arc;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

pub struct AppState {
    pub db: Pool<Postgres>,
}

pub async fn server_router() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let url_connect: String = std::env::var("CLIENT_TABLE").unwrap();

    let pool: Pool<Postgres> = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&url_connect)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the BLIZKO database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let app_state: Arc<AppState> = Arc::new(AppState { db: pool.clone() });
    
    // ? Create table
    match create_table(&app_state.db).await {
        Ok(result) => {
            println!("✅ {}", result);
            true
        }
        Err(err) => {
            println!("🔥 Failed to create table: {:?}", err);
            false
        }
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app: Router = Router::new()
        // ? Добавление нового клиента
        .route("/api/new_client", post(new_client))
        // ? Список всех клиентов
        .route("/api/client_list", get(handler_client_list))
        // ? Запрос одного клиента по id
        .route("/api/client_one", get(handler_client_id))
        // ? Обновление клиента
        .route("/api/update_client", put(update_client))
        // ? Добавление лота и  оплата
        .route("/api/pay_client", post(add_pay_client))
        // ? Обновление суммы лота
        .route("/api/update_pay", put(update_pay))
        // ? Список всех оплат
        .route("/api/pay_list", get(list_pay))
        //? Список всех оплат + список всех клиентов */
        .route("/api/all_client", get(all_client))
        //* Подключение cors и state */
        .with_state(app_state)
        .layer(cors);

    println!("Server started successfully at 0.0.0.0:8090");

    let listener: TcpListener = TcpListener::bind("0.0.0.0:8090").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
