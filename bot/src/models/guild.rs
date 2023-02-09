use sqlx_macros::Table;

#[derive(Table, Serialize, Deserialize, Debug, Clone)]
pub struct Guild {
    #[table(pk)]
    pub guild_id: i64,
    pub name: String,
    pub icon_url: Option<String>,
}
