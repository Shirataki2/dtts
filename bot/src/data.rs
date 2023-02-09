use std::sync::{Arc, atomic::AtomicBool};

#[derive(Clone)]
pub struct Data {
    pub pool: sqlx::PgPool,
    pub influx: influxdb::Client,
    pub is_loop_running: Arc<AtomicBool>,
}
