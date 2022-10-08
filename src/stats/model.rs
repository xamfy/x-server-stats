use std::str;

use serde::{Deserialize, Serialize};
use systemstat::{ByteSize, LoadAverage, Memory, PlatformMemory};
// use tokio_pg_mapper_derive::PostgresMapper;

extern crate chrono;

#[derive(Deserialize, Serialize, Clone, Debug)]
// #[pg_mapper(table = "stats")]
pub struct Stats {
    pub loadavg: Loadavg,
    pub cpu_usage: String,
    pub memory_usage: Memory,
    // PlatformMemory
    //Memory
    // pub current_system_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct StatsResponse {
    pub result: bool,
    pub data: Stats,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Loadavg {
    pub one: f32,
    pub five: f32,
    pub fifteen: f32,
}
