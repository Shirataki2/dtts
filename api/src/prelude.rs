pub use std::{collections::HashSet, sync::Arc};

pub use actix_web::{
    web::{Data, Json, Path, Query},
    HttpRequest, HttpResponse,
};
pub use sqlx::PgPool;

pub use crate::{
    client::Client,
    config::ServerConfig,
    error::Error,
    models::*,
    routes::get_data,
    serenity,
    user::{User, UserToken},
};

pub fn get_config(req: &HttpRequest) -> Result<&Arc<ServerConfig>, Error> {
    let config = get_data::<Arc<ServerConfig>>(req)?;
    Ok(config)
}
