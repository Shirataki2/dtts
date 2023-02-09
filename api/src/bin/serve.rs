#[macro_use]
extern crate log;

use api::config;
use log::info;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    std::env::set_var("RUST_BACKTRACE", "1");

    let cfg = config::ServerConfig::load_from_env();
    debug!("Server config: {:#?}", cfg);
    info!("Starting server on {}:{}", cfg.host, cfg.port);

    api::run(cfg).await?;
    Ok(())
}
