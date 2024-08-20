use sqlx::{Pool, Postgres};

pub async fn create_table(pool: &Pool<Postgres>) -> Result<String, Box<dyn std::error::Error>> {
    let check_table: &str = "SELECT EXISTS (
    SELECT 1
    FROM pg_tables
    WHERE schemaname = 'public'
    AND tablename = 'client_table'
);";

    let row: (bool,) = sqlx::query_as(&check_table).fetch_one(pool).await?;
    let table_exists = row.0;

    if table_exists {
        return Ok("Table already exists".to_string());
    }

    if !table_exists {
        let client_table: &str = "CREATE TABLE public.client_table (
            id bigserial NOT NULL,
            name varchar NOT NULL,
            url_site varchar NOT NULL,
            data_end varchar NOT NULL,
            date_start varchar NOT NULL,
            region_client varchar NOT NULL,
            url_crm varchar NOT NULL,
            pay_company varchar NOT NULL,
            plan varchar NOT NULL,
            specific_client varchar NULL,
            account_manager varchar NOT NULL,
            specialist_ads varchar NOT NULL,
            status_ads bool NOT NULL,
            status_client bool NOT NULL,
            count_metrika int8 NOT NULL,
            direct_login varchar NOT NULL,
            call_tracking_id int8 NULL,
            center_accounting varchar NOT NULL,
            plan_click int8 NULL,
            percentage_lead float4 NOT NULL,
            created_at timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
            uniq_id text NOT NULL,
            CONSTRAINT client_table_name_key UNIQUE (name),
            CONSTRAINT client_table_pkey PRIMARY KEY (id) );";

        sqlx::query(&client_table)
            .execute(pool)
            .await
            .expect("Error creating table");

        let pay_table: &str = "CREATE TABLE public.pay_table (
            id bigserial NOT NULL,
            fk_pay_table_client_table_id bigserial NOT NULL,
            name_client varchar NOT NULL,
            sum int8 NOT NULL,
            lot varchar NOT NULL,
            created_at timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
            CONSTRAINT pay_table_lot_key UNIQUE (lot),
            CONSTRAINT pay_table_pkey PRIMARY KEY (id),
            CONSTRAINT pay_table_fk_pay_table_client_table_id_fkey FOREIGN KEY (fk_pay_table_client_table_id) REFERENCES public.client_table(id) );";

        sqlx::query(&pay_table)
            .execute(pool)
            .await
            .expect("Error creating table");

        let note_client: &str = "
            CREATE TABLE public.note_client (
            id bigserial NOT NULL,
            fk_note_client_client_table_id bigserial NOT NULL,
            note text NULL,
            created_at timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
            name_client varchar NOT NULL,
            CONSTRAINT note_client_pkey PRIMARY KEY (id) ) ;";

        sqlx::query(&note_client)
            .execute(pool)
            .await
            .expect("Error creating table");
    }

    Ok("Table created successfully!".to_string()) 
}
