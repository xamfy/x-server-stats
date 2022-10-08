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
    pub memory_usage: MemoryRef,
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

#[derive(Deserialize, Serialize, Clone, Debug)]
#[cfg(target_os = "macos")]
pub struct PlatformMemoryDef {
    pub total: String,
    pub active: String,
    pub inactive: String,
    pub wired: String,
    pub free: String,
    pub purgeable: String,
    pub speculative: String,
    pub compressor: String,
    pub throttled: String,
    pub external: String,
    pub internal: String,
    pub uncompressed_in_compressor: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[cfg(target_os = "linux")]
pub struct PlatformMemoryDef {
    pub active: String,

    pub active_anon: String,

    pub active_file: String,

    pub anon_huge_pages: String,

    pub anon_pages: String,

    pub bounce: String,

    pub buffers: String,
    pub cached: String,

    pub commit_limit: String,

    pub committed_as: String,

    pub direct_map1g: String,

    pub direct_map2m: String,

    pub direct_map4k: String,

    pub dirty: String,

    pub file_huge_pages: String,

    pub file_pmd_mapped: String,

    pub hardware_corrupted: String,

    pub hugepagesize: String,

    pub hugetlb: String,

    pub inactive: String,

    pub inactive_anon: String,

    pub inactive_file: String,

    pub kreclaimable: String,

    pub kernel_stack: String,

    pub mapped: String,

    pub mem_available: String,

    pub mem_free: String,

    pub mem_total: String,

    pub mlocked: String,

    pub nfs_unstable: String,

    pub page_tables: String,

    pub percpu: String,

    pub sreclaimable: String,

    pub sunreclaim: String,

    pub shmem: String,

    pub shmem_huge_pages: String,

    pub shmem_pmd_mapped: String,

    pub slab: String,

    pub swap_cached: String,

    pub swap_free: String,
    pub swap_total: String,

    pub unevictable: String,
    pub vmalloc_chunk: String,
    pub vmalloc_total: String,
    pub vmalloc_used: String,
    pub writeback: String,
    pub writeback_tmp: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MemoryRef {
    pub total: String,
    pub free: String,
    pub platform_memory: PlatformMemoryDef,
}
