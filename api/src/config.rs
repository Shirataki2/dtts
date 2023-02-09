#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub base_url: String,
    pub web_base_url: String,
    pub discord_bot_token: String,
    pub discord_client_id: String,
    pub discord_client_secret: String,
    pub discord_auth_url: String,
    pub discord_token_url: String,
    pub discord_redirect_uri: String,
    pub gcp_tts_api_key: String,
    pub sentry_entrypoint: String,
    pub host: String,
    pub port: u16,
    pub redis_url: String,
    pub redis_key: String,
    pub stripe_secret_key: String,
}

impl ServerConfig {
    pub fn load_from_env() -> Self {
        Self {
            base_url: std::env::var("API_BASE_URL").expect("API_BASE_URL must be set"),
            web_base_url: std::env::var("WEB_BASE_URL").expect("WEB_BASE_URL must be set"),
            discord_bot_token: std::env::var("DISCORD_BOT_TOKEN")
                .expect("DISCORD_BOT_TOKEN must be set"),
            discord_client_id: std::env::var("DISCORD_BOT_CLIENT_ID")
                .expect("DISCORD_CLIENT_ID must be set"),
            discord_client_secret: std::env::var("DISCORD_BOT_CLIENT_SECRET")
                .expect("DISCORD_CLIENT_SECRET must be set"),
            discord_auth_url: std::env::var("DISCORD_BOT_AUTH_URL")
                .unwrap_or_else(|_| "https://discord.com/api/oauth2/authorize".to_string()),
            discord_token_url: std::env::var("DISCORD_BOT_TOKEN_URL")
                .unwrap_or_else(|_| "https://discord.com/api/oauth2/token".to_string()),
            discord_redirect_uri: std::env::var("DISCORD_BOT_REDIRECT_URI").unwrap_or_else(|_| {
                format!("{}/oauth2/redirect", std::env::var("API_BASE_URL").unwrap())
            }),
            gcp_tts_api_key: std::env::var("GCP_TTS_API_KEY").unwrap_or_else(|_| "".to_string()), // TODO: remove this default
            sentry_entrypoint: std::env::var("SENTRY_ENTRYPOINT")
                .expect("SENTRY_ENTRYPOINT must be set"),
            host: std::env::var("API_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("API_PORT")
                .unwrap_or_else(|_| "5000".to_string())
                .parse()
                .expect("API_PORT must be a valid port number"),
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://redis:6379".to_string()),
            redis_key: std::env::var("REDIS_KEY").expect("REDIS_KEY must be set"),
            stripe_secret_key: std::env::var("STRIPE_SECRET_KEY")
                .unwrap_or_else(|_| "".to_string()), // TODO: remove this default
        }
    }
}
