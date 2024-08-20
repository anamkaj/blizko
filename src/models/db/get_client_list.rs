use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use sqlx::prelude::FromRow;
use sqlx::PgPool;

#[derive(Debug, Clone, PartialEq, Serialize, FromRow)]
pub struct ClientTableList {
    pub id: i64,
    pub name: String,
    pub url_site: String,
    pub data_end: String,
    pub date_start: String,
    pub region_client: String,
    pub url_crm: String,
    pub pay_company: String,
    pub plan: String,
    pub specific_client: String,
    pub account_manager: String,
    pub specialist_ads: String,
    pub status_ads: bool,
    pub status_client: bool,
    pub count_metrika: i64,
    pub direct_login: String,
    pub call_tracking_id: i64,
    pub center_accounting: String,
    pub plan_click: i64,
    pub percentage_lead: f32,
    pub created_at: DateTime<Utc>,
    pub uniq_id: String,
}

impl ClientTableList {
    pub async fn all_clients(pool: PgPool) -> Result<Vec<ClientTableList>, sqlx::Error> {
        let get_all_clients: &str = "SELECT * FROM client_table ORDER BY id DESC";

        let all_clients: Vec<ClientTableList> =
            sqlx::query_as(&get_all_clients).fetch_all(&pool).await?;

        Ok(all_clients)
    }

    pub async fn get_client_id(id: i64, pool: PgPool) -> Result<ClientTableList, sqlx::Error> {
        let get_all_clients: &str = "SELECT * FROM client_table WHERE id = $1";

        let data: ClientTableList = sqlx::query_as(&get_all_clients)
            .bind(id)
            .fetch_one(&pool)
            .await?;

        Ok(data)
    }
}
