use crate::prelude::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct ServerListResponse {
    pub invited: Vec<crate::GuildInfo>,
    pub invitable: Vec<crate::GuildInfo>,
    pub other: Vec<crate::GuildInfo>,
}

#[get("/list")]
pub async fn list(req: HttpRequest, token: UserToken) -> Result<HttpResponse, Error> {
    let token = token.inner();
    let pool = get_data::<PgPool>(&req)?;
    let client = Client::from_token(token)?;
    let guilds = client.list_servers().await?;
    let guild_id_list = guilds
        .iter()
        .filter_map(|s| s.id.parse::<i64>().ok())
        .collect::<Vec<_>>();
    let invited_guilds: Vec<Guild> = sqlx::query_as!(
        Guild,
        "SELECT * FROM guild WHERE guild_id = ANY($1)",
        &guild_id_list
    )
    .fetch_all(pool)
    .await?;
    let invited_guild_ids = invited_guilds
        .iter()
        .map(|g| g.guild_id)
        .collect::<HashSet<i64>>();

    let mut invited = Vec::new();
    let mut invitable = Vec::new();
    let mut other = Vec::new();
    for guild in guilds.iter() {
        if invited_guild_ids.contains(&(guild.id.parse::<i64>().unwrap_or(0))) {
            invited.push(guild.clone());
        } else if is_invitable(guild) {
            invitable.push(guild.clone());
        } else {
            other.push(guild.clone());
        }
    }

    Ok(HttpResponse::Ok().json(ServerListResponse {
        invited,
        invitable,
        other,
    }))
}

fn is_invitable(guild: &crate::GuildInfo) -> bool {
    let is_owner = guild.owner;

    let permissions = guild.permissions.parse::<u64>().unwrap_or_default();
    let is_admin = permissions & 0x8 == 0x8;
    let is_manage_guild = permissions & 0x20 == 0x20;
    is_owner || is_admin || is_manage_guild
}
