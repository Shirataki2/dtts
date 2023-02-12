#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate maplit;

pub mod commands;
pub mod data;
pub mod error;
pub mod handler;
pub mod logs;
pub mod models;
pub mod tasks;

use std::sync::{atomic::AtomicBool, Arc};

use data::Data;
use handler::handle_event;
use poise::{serenity_prelude as serenity, Framework, FrameworkOptions};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

/// Registers or unregisters application commands in this guild or globally
#[poise::command(prefix_command, hide_in_help)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            error!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                error!("Error while handling error: {}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    std::env::set_var("RUST_BACKTRACE", "1");

    let bot_token = std::env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN is not set");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");

    let influx_client = logs::create_influx_client()?;

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(15)
        .connect(&database_url)
        .await?;

    let owner_id = std::env::var("DISCORD_BOT_OWNER_ID")
        .expect("DISCORD_BOT_OWNER_ID is not set")
        .parse::<u64>()
        .expect("DISCORD_BOT_OWNER_ID is not a valid u64");
    let owner_id = serenity::UserId(owner_id);
    let owners = hashset! { owner_id };

    let framework = Framework::builder()
        .client_settings(songbird::serenity::register)
        .options(FrameworkOptions {
            commands: vec![register(), commands::test(), commands::help()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!!".to_owned()),
                ..Default::default()
            },
            owners,
            on_error: |err| Box::pin(on_error(err)),
            event_handler: handle_event,
            ..Default::default()
        })
        .token(bot_token)
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .setup(|_ctx, _ready, _framework| {
            Box::pin(async move {
                Ok(Data {
                    pool: pool.clone(),
                    influx: influx_client.clone(),
                    is_loop_running: Arc::new(AtomicBool::new(false)),
                })
            })
        });
    framework.run().await?;
    Ok(())
}
