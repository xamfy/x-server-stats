use serde::Deserializer;
use serde::Serializer;
use std::str;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DeserializeAs, SerializeAs};

use systemstat::{ByteSize, Memory, PlatformMemory};

extern crate chrono;

#[cfg(target_os = "linux")]
pub use std::collections::BTreeMap;

#[derive(Deserialize, Serialize, Clone, Debug)]

pub struct Stats {
    pub loadavg: Loadavg,
    pub cpu_usage: String,
    pub memory_usage: MemoryWrapper,
}

#[derive(Serialize, Clone, Deserialize)]
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

// Serde calls this the definition of the remote type. It is just a copy of the
// remote data structure. The `remote` attribute gives the path to the actual
// type we intend to derive code for.
#[derive(Serialize, Clone, Deserialize)]
#[serde(remote = "PlatformMemory")]
#[cfg(target_os = "macos")]
struct PlatformMemoryDef {
    #[serde(with = "ByteSizeRef")]
    pub total: ByteSize,
    #[serde(with = "ByteSizeRef")]
    pub active: ByteSize,
    #[serde(with = "ByteSizeRef")]
    pub inactive: ByteSize,
    #[serde(with = "ByteSizeRef")]
    pub wired: ByteSize,
    #[serde(with = "ByteSizeRef")]
    pub free: ByteSize,
    #[serde(with = "ByteSizeRef")]
    pub purgeable: ByteSize,
    #[serde(with = "ByteSizeRef")]
    pub speculative: ByteSize,
    #[serde(with = "ByteSizeRef")]
    pub compressor: ByteSize,
    #[serde(with = "ByteSizeRef")]
    pub throttled: ByteSize,
    #[serde(with = "ByteSizeRef")]
    pub external: ByteSize,
    #[serde(with = "ByteSizeRef")]
    pub internal: ByteSize,
    #[serde(with = "ByteSizeRef")]
    pub uncompressed_in_compressor: ByteSize,
}

#[serde_as]
#[derive(Serialize, Clone, Deserialize)]
#[serde(remote = "PlatformMemory")]
#[cfg(target_os = "linux")]
struct PlatformMemoryDef {
    #[serde_as(as = "BTreeMap<_, ByteSizeRef>")]
    meminfo: BTreeMap<String, ByteSize>,
}

#[derive(Serialize, Clone, Deserialize)]
#[serde(remote = "ByteSize")]
struct ByteSizeRef(u64);

#[derive(Serialize, Clone, Deserialize)]
#[serde(remote = "Memory")]
pub struct MemoryRef {
    #[serde(with = "ByteSizeRef")]
    pub total: ByteSize,
    #[serde(with = "ByteSizeRef")]
    pub free: ByteSize,
    #[serde(with = "PlatformMemoryDef")]
    pub platform_memory: PlatformMemory,
}

// Ref::https://serde.rs/remote-derive.html

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct MemoryWrapper {
    #[serde(with = "MemoryRef")]
    pub memory_usage: Memory,
}

impl SerializeAs<ByteSize> for ByteSizeRef {
    fn serialize_as<S>(source: &ByteSize, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "ByteSizeRef")] &'a ByteSize);
        Helper(source).serialize(serializer)
    }
}
impl<'de> DeserializeAs<'de, ByteSize> for ByteSizeRef {
    fn deserialize_as<D>(deserializer: D) -> Result<ByteSize, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper(#[serde(with = "ByteSizeRef")] ByteSize);
        let helper = Helper::deserialize(deserializer)?;
        let Helper(v) = helper;
        Ok(v)
    }
}
