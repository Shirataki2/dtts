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
    pub session_id: String,
    pub price_id: String,
    pub subscription_id: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}
