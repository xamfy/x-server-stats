use serde::Serializer;
use std::fmt;
use std::marker::PhantomData;
use std::str;

use serde::{Deserialize, Serialize};
// use serde_with::{serde_as, DeserializeAs, SerializeAs};

use serde::de::{Deserializer, MapAccess, Visitor};
use serde::ser::SerializeMap;
use systemstat::{ByteSize, Memory, PlatformMemory};

extern crate chrono;

// #[cfg(target_os = "linux")]
pub use std::collections::BTreeMap;
// pub use std::collections::BTreeMap as MyMap;

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

// #[serde_as]
#[derive(Serialize, Clone, Deserialize)]
#[serde(remote = "PlatformMemory")]
#[cfg(target_os = "linux")]
struct PlatformMemoryDef {
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
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
    #[serde(flatten)]
    //The flatten attribute inlines keys from a field into the parent struct.
    pub memory_usage: Memory,
}

#[derive(Copy, Clone)]
pub struct BTreeWrapper {
    myMap: BTreeMap<String, ByteSize>,
}

impl Serialize for BTreeWrapper
// where
//     K: Serialize,
//     V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.myMap.len()))?;
        for (k, v) in self.myMap {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

// https://clubrobotinsat.github.io/librobot/serde/trait.Serializer.html#method.collect_map
// https://lucumr.pocoo.org/2021/11/14/abusing-serde/
// https://github.com/serde-rs/json/issues/343

// https://stackoverflow.com/questions/63846516/using-serde-json-to-serialise-maps-with-non-string-keys
// https://github.com/serde-rs/serde/issues/1387
// https://github.com/serde-rs/serde/issues/2294
// https://serde.rs/deserialize-struct.html
// https://www.anycodings.com/1questions/2410879/using-serdejson-to-serialise-maps-with-non-string-keys
// https://serde.rs/deserialize-map.html
