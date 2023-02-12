use crate::prelude::*;

#[get("")]
pub async fn index(
    req: HttpRequest,
    _user: User, // for auth
    query: Query<ServerDetailQuery>,
) -> Result<HttpResponse, Error> {
    let config = get_config(&req)?;
    let bot_token = config.discord_bot_token.clone();
    let client = Client::from_bot_token(&bot_token)?;
    let guild = match client.get_server(query.id).await? {
        Some(guild) => guild,
        None => return Err(Error::not_found("Guild")),
    };
    Ok(HttpResponse::Ok().json(guild))
}
