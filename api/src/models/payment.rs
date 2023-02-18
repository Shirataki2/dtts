use sqlx_macros::Table;

#[derive(Table, Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    #[table(pk)]
    pub id: i64,
    pub customer_id: Option<String>,
}

#[derive(Table, Serialize, Deserialize, Debug, Clone)]
pub struct Payment {
    #[table(pk)]
    pub account_id: i64,
    pub name: String,
    pub price: i32,
    pub session_id: String,
    pub price_id: String,
    pub subscription_id: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

impl Payment {
    pub async fn get_user_payments(
        pool: &sqlx::PgPool,
        user_id: i64,
    ) -> Result<Vec<Payment>, sqlx::Error> {
        sqlx::query_as!(
            Payment,
            r#"
            SELECT * FROM payment WHERE account_id = $1
            "#,
            user_id
        )
        .fetch_all(pool)
        .await
    }
}
