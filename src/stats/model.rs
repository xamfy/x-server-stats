use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
extern crate chrono;

#[derive(Deserialize, PostgresMapper, Serialize, Clone, Debug)]
#[pg_mapper(table = "stats")]
pub struct Stats {
    pub loadavg: String,
    pub cpu_usage: String,
    pub memory_usage: String,
    // pub current_system_time: NaiveDateTime,
}
