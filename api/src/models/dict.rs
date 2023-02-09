use sqlx_macros::Table;

#[derive(Table, Serialize, Deserialize, Debug, Clone)]
pub struct Dictionary {
    #[table(pk)]
    pub guild_id: i64,
    pub dict: String,
}
