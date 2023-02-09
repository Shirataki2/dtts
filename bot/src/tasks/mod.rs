use std::sync::Arc;

use poise::serenity_prelude as serenity;

use crate::{
    data::Data,
    logs::{ConnectionMetrics, GuildMetrics, UserMetrics},
};

pub async fn save_matrics(
    ctx: Arc<serenity::Context>,
    shard_mgr: Arc<tokio::sync::Mutex<serenity::ShardManager>>,
    data: Arc<Data>,
) {
    loop {
        // Logging guild count
        let guilds = ctx.cache.guilds().len() as i32;
        let _ = GuildMetrics::save(&data.influx, guilds).await;

        let users = ctx.cache.user_count() as i32;
        let _ = UserMetrics::save(&data.influx, users).await;

        // Logging latency
        {
            let shard_mgr = shard_mgr.lock().await;
            let runners = shard_mgr.runners.lock().await;
            let latencies = runners
                .iter()
                .filter_map(|(_, runner)| runner.latency)
                .collect::<Vec<_>>();
            if !latencies.is_empty() {
                let mean_latency =
                    latencies.iter().sum::<std::time::Duration>() / latencies.len() as u32;
                let latency_ms = mean_latency.as_millis() as i32;
                if let Err(e) = ConnectionMetrics::save(&data.influx, latency_ms).await {
                    error!("Failed to save metrics: {}", e);
                }
            }
        }

        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}
