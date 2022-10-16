use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CPUUsageDef {
    pub user: f32,
    pub nice: f32,
    pub system: f32,
    pub interrupt: f32,
    pub idle: f32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CPUDetails {
    pub cpu_usage: CPUUsageDef,
    pub cpu_temp: f32,
}
