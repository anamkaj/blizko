use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct AllData {
    pub id: i64,
    pub name: String,
    pub url_site: String,
    pub data_end: String,
    pub date_start: String,
    pub url_crm: String,
    pub region_client: String,
    pub pay_company: String,
    pub specific_client: String,
    pub account_manager: String,
    pub specialist_ads: String,
    pub status_ads: bool,
    pub status_client: bool,
    pub uniq_id: String,
    pub count_metrika: i64,
    pub direct_login: String,
    pub created_at: DateTime<Utc>,
    pub plan: String,
    pub center_accounting: String,
    pub plan_click: Option<i64>,
    pub percentage_lead: f32,
    pub call_tracking_id: Option<i64>,
    pub sum: i64,
    pub lot: String,
    pub pay_id: i64,
}

impl AllData {
    pub async fn all_data(pool: PgPool) -> Result<Vec<AllData>, sqlx::Error> {
        let q: &str = "
        WITH LatestPayments AS (
    SELECT
        client_table.*,
        pay_table.id AS pay_id,
        pay_table.sum,
        pay_table.lot,
        pay_table.created_at,
        ROW_NUMBER() OVER (PARTITION BY client_table.id ORDER BY pay_table.created_at DESC) AS rn
    FROM
        client_table
    JOIN
        pay_table
    ON
        client_table.id = pay_table.fk_pay_table_client_table_id
)
    SELECT
        *
    FROM
        LatestPayments
    WHERE
        rn = 1;";

        let data: Vec<AllData> = sqlx::query_as(&q).fetch_all(&pool).await?;

        Ok(data)
    }
}
