use crate::prelude::*;

#[get("/perms")]
pub async fn get_perms(
    req: HttpRequest,
    _user: User, // for auth
    query: Query<ServerDetailQuery>,
) -> Result<HttpResponse, Error> {
    let pool = get_data::<PgPool>(&req)?;
    let perms = ServerPermission::get_by_guild_id(pool, query.id).await?;
    Ok(HttpResponse::Ok().json(perms))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerPermissionBody {
    pub tag: String,
    pub permission_bit: i64,
}

#[patch("/perms")]
pub async fn patch_perms(
    req: HttpRequest,
    user: Member,
    query: Query<ServerDetailQuery>,
    body: Json<Vec<ServerPermissionBody>>,
) -> Result<HttpResponse, Error> {
    let client = get_bot_client(&req)?;
    let guild_id = query.id;
    if !user.is_server_mod(guild_id, &client).await {
        return Err(Error::forbidden("You are not a server moderator!"));
    }
    let pool = get_data::<PgPool>(&req)?;
    let tag_perms = body
        .iter()
        .map(|p| (p.tag.clone(), p.permission_bit))
        .collect::<Vec<(String, i64)>>();
    if tag_perms.is_empty() {
        return Err(Error::bad_request("No permissions provided"));
    }
    let perms = ServerPermission::multiple_insert(pool, guild_id, tag_perms).await?;
    Ok(HttpResponse::Ok().json(perms))
}

#[patch("/perms")]
pub async fn test(
    _req: HttpRequest,
    _user: Member,
    _query: Query<ServerDetailQuery>,
    body: Bytes,
) -> Result<HttpResponse, Error> {
    println!("{:?}", body);
    Ok(HttpResponse::Ok().finish())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerMemberPermissionCheckBody {
    pub is_mod: bool,
}

#[get("/perms/check")]
pub async fn check_perms(
    req: HttpRequest,
    user: Member,
    query: Query<ServerDetailQuery>,
) -> Result<HttpResponse, Error> {
    let client = get_bot_client(&req)?;
    let guild_id = query.id;
    let is_mod = user.is_server_mod(guild_id, &client).await;
    let body = ServerMemberPermissionCheckBody { is_mod };
    Ok(HttpResponse::Ok().json(body))
}
