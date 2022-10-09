use crate::stats::{Loadavg, MemoryWrapper};

extern crate minify;

extern crate systemstat;

use systemstat::{ByteSize, Memory, PlatformMemory};

pub fn get_load(one: f32, five: f32, fifteen: f32) -> Loadavg {
    Loadavg { one, five, fifteen }
}

pub fn get_empty_platform_memory() -> PlatformMemory {
    let empty_error_byte: ByteSize = ByteSize::mb(0);
    PlatformMemory {
        total: empty_error_byte,
        active: empty_error_byte,
        inactive: empty_error_byte,
        wired: empty_error_byte,
        free: empty_error_byte,
        purgeable: empty_error_byte,
        speculative: empty_error_byte,
        compressor: empty_error_byte,
        throttled: empty_error_byte,
        external: empty_error_byte,
        internal: empty_error_byte,
        uncompressed_in_compressor: empty_error_byte,
    }
}

pub fn get_empty_memory_wrapper() -> MemoryWrapper {
    MemoryWrapper {
        memory_usage: get_empty_memory_usage(),
    }
}

pub fn get_empty_memory_usage() -> Memory {
    let empty_error_byte: ByteSize = ByteSize::mb(0);
    Memory {
        total: empty_error_byte,
        free: empty_error_byte,
        platform_memory: get_empty_platform_memory(),
    }
}
