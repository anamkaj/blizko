use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use validator::Validate;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromRow, Validate)]
pub struct PayTable {
    pub id: Option<i64>,
    pub fk_pay_table_client_table_id: Option<i64>,
    pub name_client: String,
    pub sum: i64,
    pub lot: String,
    pub created_at: Option<DateTime<Utc>>,
}
//* Добавление нового лота */
impl PayTable {
    pub async fn add_new_pay(&self, pool: PgPool) -> Result<PayTable, sqlx::Error> {
        let add_pay: &str = "INSERT INTO pay_table
        (
        fk_pay_table_client_table_id,
        name_client,
        sum,
        lot,
        created_at
        ) 
        VALUES (
        $1,$2,$3,$4,CURRENT_TIMESTAMP )
        RETURNING *;";

        let data: PayTable = sqlx::query_as(&add_pay)
            .bind(&self.id)
            .bind(&self.name_client)
            .bind(&self.sum)
            .bind(&self.lot)
            .fetch_one(&pool)
            .await?;

        Ok(data)
    }
    //* Обновление суммы лота */
    pub async fn update_pay(&self, pool: PgPool) -> Result<PayTable, sqlx::Error> {
        let update_sum_query: &str = "UPDATE pay_table
                              SET sum = $2
                              WHERE id= $1
                              RETURNING *;";

        let data: PayTable = sqlx::query_as(&update_sum_query)
            .bind(&self.id)
            .bind(&self.sum)
            .fetch_one(&pool)
            .await?;

        Ok(data)
    }

    pub async fn get_all_pay(pool: PgPool) -> Result<Vec<PayTable>, sqlx::Error> {
        let add_pay: &str = "SELECT * FROM pay_table ORDER BY id DESC";

        let data: Vec<PayTable> = sqlx::query_as(&add_pay).fetch_all(&pool).await?;

        Ok(data)
    }
}
