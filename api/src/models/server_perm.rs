use sqlx::PgPool;
use sqlx_macros::Table;

#[derive(Table, Serialize, Deserialize, Debug, Clone)]
#[table(name = "server_permission")]
pub struct ServerPermission {
    #[table(pk)]
    pub guild_id: i64,
    pub tag: String,
    pub permission_bit: i64,
}

impl ServerPermission {
    pub async fn get_by_guild_id(
        pool: &PgPool,
        guild_id: i64,
    ) -> Result<Vec<ServerPermission>, sqlx::Error> {
        sqlx::query_as!(
            ServerPermission,
            r#"
            SELECT * FROM server_permission WHERE guild_id = $1
            "#,
            guild_id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn multiple_insert(
        pool: &PgPool,
        guild_id: i64,
        tag_perms: Vec<(String, i64)>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let mut perms_list = vec![];
        for (tag, perm) in tag_perms {
            let p = sqlx::query_as!(
                ServerPermission,
                r#"
                INSERT INTO server_permission (guild_id, tag, permission_bit)
                VALUES ($1, $2, $3)
                ON CONFLICT (guild_id, tag) DO UPDATE
                SET permission_bit = $3
                RETURNING *
                "#,
                guild_id,
                tag,
                perm
            )
            .fetch_one(pool)
            .await?;
            perms_list.push(p);
        }
        Ok(perms_list)
    }
}
