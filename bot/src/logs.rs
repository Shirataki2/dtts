use chrono::{DateTime, Utc};
use influxdb::InfluxDbWriteable;

#[derive(InfluxDbWriteable, Serialize, Deserialize, Debug, Clone)]
pub struct GuildMetrics {
    pub time: DateTime<Utc>,
    pub guilds: i32,
}

impl GuildMetrics {
    pub async fn save(client: &influxdb::Client, guilds: i32) -> Result<(), anyhow::Error> {
        let data = Self {
            time: chrono::Utc::now(),
            guilds,
        };
        client.query(data.into_query("guilds")).await?;
        Ok(())
    }
}

#[derive(InfluxDbWriteable, Serialize, Deserialize, Debug, Clone)]
pub struct UserMetrics {
    pub time: DateTime<Utc>,
    pub users: i32,
}

impl UserMetrics {
    pub async fn save(client: &influxdb::Client, users: i32) -> Result<(), anyhow::Error> {
        let data = Self {
            time: chrono::Utc::now(),
            users,
        };
        client.query(data.into_query("users")).await?;
        Ok(())
    }
}

#[derive(InfluxDbWriteable, Serialize, Deserialize, Debug, Clone)]
pub struct ConnectionMetrics {
    pub time: DateTime<Utc>,
    pub ping: i32,
}

impl ConnectionMetrics {
    pub async fn save(client: &influxdb::Client, ping: i32) -> Result<(), anyhow::Error> {
        let data = Self {
            time: chrono::Utc::now(),
            ping,
        };
        client.query(data.into_query("connections")).await?;
        Ok(())
    }
}

pub fn create_influx_client() -> Result<influxdb::Client, anyhow::Error> {
    let mut header = reqwest::header::HeaderMap::new();
    header.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&format!(
            "Token {}",
            std::env::var("DOCKER_INFLUXDB_INIT_ADMIN_TOKEN").unwrap()
        ))?,
    );
    let influx_client = reqwest::Client::builder().default_headers(header).build()?;

    let influx_client = influxdb::Client::new(
        "http://influxdb:8086",
        std::env::var("DOCKER_INFLUXDB_INIT_BUCKET").unwrap(),
    )
    .with_http_client(influx_client);
    Ok(influx_client)
}
