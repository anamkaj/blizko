use crate::utils::db_utils::generation_uniq_id;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use sqlx::prelude::FromRow;
use sqlx::PgPool;
use validator::Validate;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromRow, Validate)]
pub struct AddNewClientTable {
    pub id: Option<i64>,
    #[validate(length(min = 3))]
    pub name: String,
    #[validate(url)]
    pub url_site: String,
    #[validate(length(min = 3))]
    pub date_start: String,
    #[validate(length(min = 3))]
    pub data_end: String,
    #[validate(url)]
    pub url_crm: String,
    pub region_client: String,
    pub pay_company: String,
    pub specific_client: Option<String>,
    pub account_manager: String,
    pub specialist_ads: String,
    pub status_ads: bool,
    pub status_client: bool,
    pub count_metrika: i64,
    pub direct_login: String,
    pub created_at: Option<DateTime<Utc>>,
    pub plan: String,
    #[validate(length(min = 2))]
    pub center_accounting: String,
    pub plan_click: Option<i64>,
    pub percentage_lead: f32,
    pub call_tracking_id: Option<i64>,
}

impl AddNewClientTable {
    pub async fn add_to_db(&self, pool: PgPool) -> Result<AddNewClientTable, sqlx::Error> {
        let id_uniq: String = generation_uniq_id().await.unwrap();
        let add_client: &str = "INSERT INTO client_table
        (
        name, url_site, date_start, data_end, url_crm, region_client, pay_company, specific_client, account_manager, specialist_ads, status_ads, status_client, uniq_id, count_metrika, direct_login, plan, center_accounting, plan_click, percentage_lead, call_tracking_id
        ) 
        VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20) 
        RETURNING *;";

        let data: AddNewClientTable = sqlx::query_as(&add_client)
            .bind(&self.name)
            .bind(&self.url_site)
            .bind(&self.date_start)
            .bind(&self.data_end)
            .bind(&self.url_crm)
            .bind(&self.region_client)
            .bind(&self.pay_company)
            .bind(&self.specific_client)
            .bind(&self.account_manager)
            .bind(&self.specialist_ads)
            .bind(self.status_ads)
            .bind(self.status_client)
            .bind(id_uniq)
            .bind(self.count_metrika)
            .bind(&self.direct_login)
            .bind(&self.plan)
            .bind(&self.center_accounting)
            .bind(&self.plan_click)
            .bind(&self.percentage_lead)
            .bind(&self.call_tracking_id)
            .fetch_one(&pool)
            .await?;

        println!("Новый клиент добавлен в базу данных: {:#?}", &self.name);
        Ok(data)
    }
}
