use crate::stats::{CPUDetails, CPUUsageDef, Loadavg, MemoryRef, PlatformMemoryDef, StatsResponse};
use crate::utils::{get_empty_cpu_details, get_empty_memory_usage, get_load, has_key};

use crate::Stats;
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{get, Error, HttpResponse, Responder};
use actix_web_lab::__reexports::serde_json;
use askama::Template;
use core::fmt;
use minify::html::minify;
use std::fmt::Display;
extern crate minify;

extern crate systemstat;
use std::thread;
use std::time::Duration;
use systemstat::platform::PlatformImpl;
use systemstat::{Platform, System};

impl Display for Loadavg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Load avg: ")
    }
}

impl Display for MemoryRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.platform_memory)
    }
}

impl Display for CPUDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CPU load: {}% user, {}% nice, {}% system, {}% intr, {}% idle, {} temp",
            self.cpu_usage.user * 100.0,
            self.cpu_usage.nice * 100.0,
            self.cpu_usage.system * 100.0,
            self.cpu_usage.interrupt * 100.0,
            self.cpu_usage.idle * 100.0,
            self.cpu_temp
        )
    }
}

#[derive(Template)] // this will generate the code...
#[template(path = "index.html")] // using the template in this path, relative
                                 // to the `templates` dir in the crate root
struct StatsTemplate {
    stats: StatsResponse,
}

impl Responder for StatsResponse {
    type Body = BoxBody;
    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let res_body = serde_json::to_string(&self).unwrap();

        // Create HttpResponse and set Content Type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(res_body)
    }
}

pub async fn get_stats_from_linux(sys: PlatformImpl) -> StatsResponse {
    match sys.cpu_load_aggregate() {
        Ok(cpu) => {
            println!("\nMeasuring CPU load...");
            thread::sleep(Duration::from_secs(1));
            let cpu = cpu.done().unwrap();

            let load_avg = match sys.load_average() {
                Ok(load) => Loadavg {
                    one: load.one,
                    five: load.five,
                    fifteen: load.fifteen,
                },
                Err(_) => get_load(0.0, 0.0, 0.0),
            };

            let memory_details = match sys.memory() {
                Ok(mem) => {
                    let platform_memory = PlatformMemoryDef {
                        active: has_key(&mem, "Active"),
                        active_anon: has_key(&mem, "Active(anon)"),
                        active_file: has_key(&mem, "Active(file)"),
                        anon_huge_pages: has_key(&mem, "AnonHugePages"),
                        anon_pages: has_key(&mem, "AnonPages"),
                        bounce: has_key(&mem, "Bounce"),
                        buffers: has_key(&mem, "Buffers"),
                        cached: has_key(&mem, "Cached"),
                        commit_limit: has_key(&mem, "CommitLimit"),
                        committed_as: has_key(&mem, "Committed_AS"),
                        direct_map1g: has_key(&mem, "DirectMap1G"),
                        direct_map2m: has_key(&mem, "DirectMap2M"),
                        direct_map4k: has_key(&mem, "DirectMap4k"),
                        dirty: has_key(&mem, "Dirty"),
                        file_huge_pages: has_key(&mem, "FileHugePages"),
                        file_pmd_mapped: has_key(&mem, "FilePmdMapped"),
                        hardware_corrupted: has_key(&mem, "HardwareCorrupted"),
                        hugepagesize: has_key(&mem, "Hugepagesize"),
                        hugetlb: has_key(&mem, "Hugetlb"),
                        inactive: has_key(&mem, "Inactive"),
                        inactive_anon: has_key(&mem, "Inactive(anon)"),
                        inactive_file: has_key(&mem, "Inactive(file)"),
                        kreclaimable: has_key(&mem, "KReclaimable"),
                        kernel_stack: has_key(&mem, "KernelStack"),
                        mapped: has_key(&mem, "Mapped"),
                        mem_available: has_key(&mem, "MemAvailable"),
                        mem_free: has_key(&mem, "MemFree"),
                        mem_total: has_key(&mem, "MemTotal"),
                        mlocked: has_key(&mem, "Mlocked"),
                        nfs_unstable: has_key(&mem, "NFS_Unstable"),
                        page_tables: has_key(&mem, "PageTables"),
                        percpu: has_key(&mem, "Percpu"),
                        sreclaimable: has_key(&mem, "SReclaimable"),
                        sunreclaim: has_key(&mem, "SUnreclaim"),
                        shmem: has_key(&mem, "Shmem"),
                        shmem_huge_pages: has_key(&mem, "ShmemHugePages"),
                        shmem_pmd_mapped: has_key(&mem, "ShmemPmdMapped"),
                        slab: has_key(&mem, "Slab"),
                        swap_cached: has_key(&mem, "SwapCached"),
                        swap_free: has_key(&mem, "SwapFree"),
                        swap_total: has_key(&mem, "SwapTotal"),
                        unevictable: has_key(&mem, "Unevictable"),
                        vmalloc_chunk: has_key(&mem, "VmallocChunk"),
                        vmalloc_total: has_key(&mem, "VmallocTotal"),
                        vmalloc_used: has_key(&mem, "VmallocUsed"),
                        writeback: has_key(&mem, "Writeback"),
                        writeback_tmp: has_key(&mem, "WritebackTmp"),
                    };
                    MemoryRef {
                        total: mem.total.to_string(),
                        free: mem.free.to_string(),
                        platform_memory,
                    }
                }
                Err(_) => get_empty_memory_usage(),
            };

            println!(
                "CPU load: {}% user, {}% nice, {}% system, {}% intr, {}% idle ",
                cpu.user * 100.0,
                cpu.nice * 100.0, // TG :cpu load nice
                cpu.system * 100.0,
                cpu.interrupt * 100.0,
                cpu.idle * 100.0
            );

            let cpu_temp = if let Ok(cpu_temp) = sys.cpu_temp() {
                cpu_temp
            } else {
                0.0
            };

            let stats = Stats {
                loadavg: load_avg,
                memory_usage: memory_details,
                cpu_details: CPUDetails {
                    cpu_usage: CPUUsageDef {
                        user: cpu.user,
                        nice: cpu.nice,
                        system: cpu.system,
                        interrupt: cpu.interrupt,
                        idle: cpu.idle,
                    },
                    cpu_temp,
                },
            };
            return StatsResponse {
                result: true,
                data: stats,
            };
        }
        Err(x) => println!("\nCPU load: error: {}", x),
    }
    let stats = Stats {
        loadavg: get_load(0.0, 0.0, 0.0),
        memory_usage: get_empty_memory_usage(),
        cpu_details: get_empty_cpu_details(),
    };
    StatsResponse {
        data: stats,
        result: false,
    }
}

#[get("")]
pub async fn index_page() -> Result<HttpResponse, Error> {
    let sys = System::new();

    match sys.cpu_temp() {
        Ok(cpu_temp) => println!("\nCPU temp: {}", cpu_temp),
        Err(x) => println!("\nCPU temp: {}", x),
    };

    let stats = get_stats_from_linux(sys).await;
    println!("{:?}", stats.data);

    let stats_html = StatsTemplate { stats }; // instantiate your struct

    let html_str = minify(&stats_html.render().unwrap());

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html_str))
}

#[get("/v1/stats")]
async fn status_get_api() -> impl Responder {
    let sys = System::new();
    let stats = get_stats_from_linux(sys).await;
    println!("{:?}", stats.data);

    let response = serde_json::to_string(&stats).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response)
}
