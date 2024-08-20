use dotenv::dotenv;
use sqlx::{postgres::PgRow, Pool, Postgres};
use std::error::Error;
use uuid::Uuid;

//* Проверка на дубли счетчиков , поиск по счетчикам метрики */
pub async fn search_count(id: i64, pool: Pool<Postgres>) -> Result<Option<PgRow>, Box<dyn Error>> {
    dotenv().ok();

    let check_query: &str = "SELECT id FROM counters_metrika WHERE counter_id =$1";

    let check_row: Option<PgRow> = sqlx::query(&check_query)
        .bind(id)
        .fetch_optional(&pool)
        .await
        .expect("Ошибка проверки на дубли");

    Ok(check_row)
}

//*Генерация уникальных ID */
pub async fn generation_uniq_id() -> Result<String, Box<dyn Error>> {
    let id: Uuid = Uuid::new_v4();
    Ok(id.to_string())
}
