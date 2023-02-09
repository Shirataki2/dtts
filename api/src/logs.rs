use chrono::{DateTime, Utc};
use influxdb::InfluxDbWriteable;

#[derive(InfluxDbWriteable, Serialize, Deserialize, Debug, Clone)]
pub struct AccessMetrics {
    pub time: DateTime<Utc>,
    #[influxdb(tag)]
    pub path: String,
    pub elapsed: i64,
}

impl AccessMetrics {
    pub async fn save(
        client: &influxdb::Client,
        path: String,
        elapsed: i64,
    ) -> Result<(), anyhow::Error> {
        let data = Self {
            time: chrono::Utc::now(),
            path,
            elapsed,
        };
        client.query(data.into_query("accesses")).await?;
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
