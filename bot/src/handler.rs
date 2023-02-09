use std::sync::{atomic::Ordering, Arc};

use poise::BoxFuture;

use crate::{models, serenity, Data, Error, tasks};

pub fn handle_event<'a>(
    ctx: &'a serenity::Context,
    event: &'a poise::Event<'a>,
    framework: poise::FrameworkContext<'a, Data, Error>,
    data: &'a Data,
) -> BoxFuture<'a, Result<(), Error>> {
    Box::pin(async move {
        use poise::Event::*;
        match event {
            GuildCreate { guild, .. } => {
                let pool = &data.pool;
                info!("Joined guild: {}", guild.name);
                let guild_id = guild.id.0 as i64;
                let name = guild.name.clone();
                let icon_url = guild.icon_url();
                let guild = models::guild::Guild {
                    guild_id,
                    name,
                    icon_url,
                };
                let result = guild.create(pool).await;
                // Ignore duplicate key error
                if let Err(sqlx::Error::Database(ref err)) = result {
                    let code = err.code();
                    if code != Some(std::borrow::Cow::Borrowed("23505")) {
                        result?;
                    }
                } else {
                    result?;
                }
            }
            GuildDelete {
                incomplete: guild, ..
            } => {
                let pool = &data.pool;
                info!("Left guild: {}", guild.id);
                let guild_id = guild.id.0 as i64;
                let guild = models::guild::Guild::get(pool, guild_id).await?;
                guild.delete(pool).await?;
            }
            CacheReady { .. } => {
                info!("Cache ready");
                let ctx = Arc::new(ctx.clone());
                let data2 = Arc::new(data.clone());
                if !data.is_loop_running.load(Ordering::Relaxed) {
                    info!("Spawning tasks");
                    tokio::spawn(tasks::save_matrics(ctx, framework.shard_manager(), data2));
                    data.is_loop_running.swap(true, Ordering::Relaxed);
                }
            }
            other => {
                debug!("Unhandled event: {}", other.name())
            }
        }
        Ok(())
    })
}
