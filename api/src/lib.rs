#[macro_use]
extern crate serde;
#[macro_use]
extern crate log;
#[macro_use]
extern crate actix_web;

pub mod client;
pub mod config;
pub mod error;
pub mod logs;
pub mod macros;
pub mod middleware;
pub mod models;
pub mod oauth2client;
pub mod prelude;
pub mod routes;
pub mod user;
use std::{io, sync::Arc};

use crate::middleware::influx::Influx;

pub use poise::serenity_prelude as serenity;
pub type BoxFutureResult<T, E> = std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, E>>>>;

use actix_session::{
    config::PersistentSession, storage::RedisActorSessionStore, SessionMiddleware,
};
use actix_web::{
    cookie::{time::Duration, Key},
    App, HttpServer,
};
use oauth2::{basic::BasicTokenType, EmptyExtraTokenFields, StandardTokenResponse};
pub use oauth2client::OAuth2Client;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::Mutex;

pub type AccessToken = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

pub const DISCORD_API_URL: &str = "https://discord.com/api/v10";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildInfo {
    pub id: String,
    pub icon: Option<String>,
    pub name: String,
    pub owner: bool,
    pub permissions: String,
}

#[actix_web::get("/")]
async fn index() -> String {
    String::from("Ready")
}

#[actix_web::get("/debug/failing")]
async fn fail() -> Result<String, io::Error> {
    error!("An error happens here");
    Err(io::Error::new(
        io::ErrorKind::Other,
        "An error happens here",
    ))
}

pub async fn run(cfg: config::ServerConfig) -> std::io::Result<()> {
    let cfg = Arc::new(cfg);
    let cfg2 = Arc::clone(&cfg);
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let server = HttpServer::new(move || {
        let auth = OAuth2Client::new(
            cfg2.discord_client_id.clone(),
            cfg2.discord_client_secret.clone(),
            cfg2.discord_redirect_uri.clone(),
            cfg2.discord_auth_url.clone(),
            cfg2.discord_token_url.clone(),
            vec!["identify".into(), "guilds".into()],
        );

        let redis_url = cfg2.redis_url.clone();
        let scheme_stripped = redis_url
            .strip_prefix("redis://")
            .expect("Redis URL must start with redis://")
            .to_string();
        let redis_client = redis::Client::open(redis_url).expect("Failed to connect to redis");
        let redis_client = Arc::new(Mutex::new(redis_client));
        let redis_key = Key::from(cfg2.redis_key.as_bytes());

        let stripe_client = stripe::Client::new(cfg2.stripe_secret_key.as_str());

        App::new()
            .wrap(sentry_actix::Sentry::new())
            .app_data(auth)
            .app_data(pool.clone())
            .app_data(cfg2.clone())
            .app_data(redis_client)
            .app_data(stripe_client)
            .wrap(actix_web::middleware::Logger::default())
            .wrap(
                SessionMiddleware::builder(RedisActorSessionStore::new(scheme_stripped), redis_key)
                    .cookie_http_only(false)
                    .cookie_name("access_token".into())
                    .session_lifecycle(PersistentSession::default().session_ttl(Duration::days(14)))
                    .build(),
            )
            .wrap(Influx)
            .configure(routes::init_routes)
            .service(fail)
            .service(index)
        //.default_service(web::route().to(routes::not_found))
    })
    .bind(format!("{}:{}", cfg.host, cfg.port))?;
    info!("Starting Server");
    server.run().await
}
