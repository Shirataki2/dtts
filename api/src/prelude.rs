pub use std::{collections::HashSet, sync::Arc};

pub use actix_session::Session;
pub use actix_web::{
    web::{self, Bytes, Data, Json, Path, Payload, Query},
    HttpRequest, HttpResponse,
};
pub use reqwest::StatusCode;
pub use sqlx::PgPool;

pub use crate::{
    client::Client,
    config::ServerConfig,
    error::{create_error, Error},
    models::*,
    routes::{donate::get_stripe_client, get_data, servers::perms::check_user_perms},
    serenity,
    user::{Member, User, UserToken},
    AccessToken, BoxFutureResult, OAuth2Client,
};

pub fn get_config(req: &HttpRequest) -> Result<&Arc<ServerConfig>, Error> {
    let config = get_data::<Arc<ServerConfig>>(req)?;
    Ok(config)
}

pub fn get_bot_client(req: &HttpRequest) -> Result<Client, Error> {
    let config = get_config(req)?;
    let bot_token = config.discord_bot_token.clone();
    let client = Client::from_bot_token(&bot_token)?;
    Ok(client)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerDetailQuery {
    pub id: i64,
}
