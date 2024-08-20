use serde::Deserialize;
use serde::Serialize;
use sqlx::prelude::FromRow;
use sqlx::PgPool;
use validator::Validate;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromRow, Validate)]
pub struct UpdateClientTable {
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
    pub specific_client: String,
    pub account_manager: String,
    pub specialist_ads: String,
    pub status_ads: bool,
    pub status_client: bool,
    pub count_metrika: i64,
    pub direct_login: String,
    pub uniq_id: Option<String>,
    pub plan: String,
    pub center_accounting: String,
    pub plan_click: Option<i64>,
    pub percentage_lead: f32,
    pub call_tracking_id: Option<i64>,
}

impl UpdateClientTable {
    pub async fn update_client(&self, pool: PgPool) -> Result<UpdateClientTable, sqlx::Error> {
        let update_client = "
        UPDATE client_table
        SET 
            name = $1,
            url_site = $2,
            date_start = $3,
            data_end = $4,
            url_crm = $5,
            region_client = $6,
            pay_company = $7,
            specific_client = $8,
            account_manager = $9,
            specialist_ads = $10,
            status_ads = $11,
            status_client = $12,
            count_metrika = $13,
            direct_login = $14,
            uniq_id = $15,
            plan = $16,
            center_accounting = $17,
            plan_click = $18,
            percentage_lead = $19,
            call_tracking_id = $20
        WHERE id = $21
        RETURNING *;
    ";

        let data: UpdateClientTable = sqlx::query_as(&update_client)
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
            .bind(self.count_metrika)
            .bind(&self.direct_login)
            .bind(&self.uniq_id)
            .bind(&self.plan)
            .bind(&self.center_accounting)
            .bind(&self.plan_click)
            .bind(self.percentage_lead)
            .bind(&self.call_tracking_id)
            .bind(self.id)
            .fetch_one(&pool)
            .await
            .expect("Ошибка добавления нового клиента");

        println!("Данные для клиента обновлены: {:#?}", &self.name);
        Ok(data)
    }
}
