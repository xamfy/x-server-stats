use crate::stats::{CPUDetails, CPUUsageDef};
use crate::stats::{Loadavg, MemoryRef, PlatformMemoryDef};

extern crate minify;

extern crate systemstat;

pub fn get_load(one: f32, five: f32, fifteen: f32) -> Loadavg {
    Loadavg { one, five, fifteen }
}

#[cfg(target_os = "macos")]
pub fn get_empty_platform_memory() -> PlatformMemoryDef {
    #[cfg(target_os = "macos")]
    PlatformMemoryDef {
        total: "".to_string(),
        active: "".to_string(),
        inactive: "".to_string(),
        wired: "".to_string(),
        free: "".to_string(),
        purgeable: "".to_string(),
        speculative: "".to_string(),
        compressor: "".to_string(),
        throttled: "".to_string(),
        external: "".to_string(),
        internal: "".to_string(),
        uncompressed_in_compressor: "".to_string(),
    }
}

#[cfg(target_os = "linux")]
pub fn get_empty_platform_memory() -> PlatformMemoryDef {
    PlatformMemoryDef {
        active: "".to_string(),
        active_anon: "".to_string(),
        active_file: "".to_string(),
        anon_huge_pages: "".to_string(),
        anon_pages: "".to_string(),
        bounce: "".to_string(),
        buffers: "".to_string(),
        cached: "".to_string(),
        commit_limit: "".to_string(),
        committed_as: "".to_string(),
        direct_map1g: "".to_string(),
        direct_map2m: "".to_string(),
        direct_map4k: "".to_string(),
        dirty: "".to_string(),
        file_huge_pages: "".to_string(),
        file_pmd_mapped: "".to_string(),
        hardware_corrupted: "".to_string(),
        hugepagesize: "".to_string(),
        hugetlb: "".to_string(),
        inactive: "".to_string(),
        inactive_anon: "".to_string(),
        inactive_file: "".to_string(),
        kreclaimable: "".to_string(),
        kernel_stack: "".to_string(),
        mapped: "".to_string(),
        mem_available: "".to_string(),
        mem_free: "".to_string(),
        mem_total: "".to_string(),
        mlocked: "".to_string(),
        nfs_unstable: "".to_string(),
        page_tables: "".to_string(),
        percpu: "".to_string(),
        sreclaimable: "".to_string(),
        sunreclaim: "".to_string(),
        shmem: "".to_string(),
        shmem_huge_pages: "".to_string(),
        shmem_pmd_mapped: "".to_string(),
        slab: "".to_string(),
        swap_cached: "".to_string(),
        swap_free: "".to_string(),
        swap_total: "".to_string(),
        unevictable: "".to_string(),
        vmalloc_chunk: "".to_string(),
        vmalloc_total: "".to_string(),
        vmalloc_used: "".to_string(),
        writeback: "".to_string(),
        writeback_tmp: "".to_string(),
    }
}

#[cfg(target_os = "macos")]
pub fn get_empty_memory_usage() -> MemoryRef {
    #[cfg(target_os = "macos")]
    MemoryRef {
        total: "".to_string(),
        free: "".to_string(),
        platform_memory: get_empty_platform_memory(),
    }
}

#[cfg(target_os = "linux")]
pub fn get_empty_memory_usage() -> MemoryRef {
    MemoryRef {
        total: "".to_string(),
        free: "".to_string(),
        platform_memory: get_empty_platform_memory(),
    }
}

pub fn get_empty_cpu_details() -> CPUDetails {
    CPUDetails {
        cpu_usage: CPUUsageDef {
            user: 0.0,
            nice: 0.0,
            system: 0.0,
            interrupt: 0.0,
            idle: 0.0,
        },
        cpu_temp: 0.0,
    }
}
