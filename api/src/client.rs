use anyhow::Context;
use oauth2::TokenResponse;
use reqwest::{header, StatusCode};

use crate::{check_response, error::Error, serenity, AccessToken};

#[derive(Debug, Clone)]
pub struct Client {
    inner: reqwest::Client,
}

impl Client {
    pub fn from_token(token: &AccessToken) -> Result<Self, Error> {
        let header_value = reqwest::header::HeaderValue::from_str(&format!(
            "Bearer {}",
            token.access_token().secret()
        ))
        .context("Failed to create header value")?;
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(header::AUTHORIZATION, header_value);
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        Ok(Self { inner: client })
    }

    pub fn from_bot_token(token: &str) -> Result<Self, Error> {
        let header_value = reqwest::header::HeaderValue::from_str(&format!("Bot {}", token))
            .context("Failed to create header value")?;
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(header::AUTHORIZATION, header_value);
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        Ok(Self { inner: client })
    }

    pub async fn current_user(&self) -> Result<serenity::CurrentUser, Error> {
        let url = format!("{}/users/@me", crate::DISCORD_API_URL);
        let resp = self.inner.get(&url).send().await?;
        let user = check_response!(serenity::CurrentUser: resp);
        Ok(user)
    }

    pub async fn list_servers(&self) -> Result<Vec<crate::GuildInfo>, Error> {
        let url = format!("{}/users/@me/guilds", crate::DISCORD_API_URL);
        let resp = self.inner.get(&url).send().await?;
        let guilds = check_response!(Vec<crate::GuildInfo>: resp);
        Ok(guilds)
    }

    pub async fn get_server(&self, id: u64) -> Result<Option<serenity::PartialGuild>, Error> {
        let url = format!("{}/guilds/{}", crate::DISCORD_API_URL, id);
        let resp = self.inner.get(&url).send().await?;
        if resp.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }
        let guild = check_response!(serenity::PartialGuild: resp);
        Ok(Some(guild))
    }
}

impl AsRef<reqwest::Client> for Client {
    fn as_ref(&self) -> &reqwest::Client {
        &self.inner
    }
}
