use crate::stats::{Loadavg, MemoryRef, PlatformMemoryDef, StatsResponse};
use crate::utils::{get_empty_memory_usage, get_load};

use crate::Stats;
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web_lab::__reexports::serde_json;
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use askama::Template;
use core::fmt;
use minify::html::minify;
use std::fmt::Display;


use actix::{Actor, StreamHandler};
use actix_web_actors::ws;


extern crate minify;

extern crate systemstat;
use std::thread;
use std::time::Duration;
use systemstat::platform::PlatformImpl;
use systemstat::{Memory, Platform, System};

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

#[derive(Template)] // this will generate the code...
#[template(path = "index.html")] // using the template in this path, relative
                                 // to the `templates` dir in the crate root
struct StatsTemplate {
    stats: Stats,
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

pub async fn get_stats_from_linux(sys: PlatformImpl) -> Stats {
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

            #[cfg(target_os = "macos")]
            let memory_details = match sys.memory() {
                Ok(mem) => {
                    let platform_memory = PlatformMemoryDef {
                        total: mem.total.to_string(),
                        active: mem.platform_memory.active.to_string(),
                        inactive: mem.platform_memory.inactive.to_string(),
                        wired: mem.platform_memory.wired.to_string(),
                        free: mem.platform_memory.free.to_string(),
                        purgeable: mem.platform_memory.purgeable.to_string(),
                        speculative: mem.platform_memory.speculative.to_string(),
                        compressor: mem.platform_memory.compressor.to_string(),
                        throttled: mem.platform_memory.throttled.to_string(),
                        external: mem.platform_memory.external.to_string(),
                        internal: mem.platform_memory.internal.to_string(),
                        uncompressed_in_compressor: mem
                            .platform_memory
                            .uncompressed_in_compressor
                            .to_string(),
                    };
                    MemoryRef {
                        total: mem.total.to_string(),
                        free: mem.free.to_string(),
                        platform_memory,
                    }
                }
                Err(_) => get_empty_memory_usage(),
            };

            #[cfg(target_os = "linux")]
            let memory_details = match sys.memory() {
                Ok(mem) => {
                    let platform_memory = PlatformMemoryDef {
                        active: check_whether_key(&mem, "Active"),
                        active_anon: check_whether_key(&mem, "Active(anon)"),
                        active_file: check_whether_key(&mem, "Active(file)"),
                        anon_huge_pages: check_whether_key(&mem, "AnonHugePages"),
                        anon_pages: check_whether_key(&mem, "AnonPages"),
                        bounce: check_whether_key(&mem, "Bounce"),
                        buffers: check_whether_key(&mem, "Buffers"),
                        cached: check_whether_key(&mem, "Cached"),
                        commit_limit: check_whether_key(&mem, "CommitLimit"),
                        committed_as: check_whether_key(&mem, "Committed_AS"),
                        direct_map1g: check_whether_key(&mem, "DirectMap1G"),
                        direct_map2m: check_whether_key(&mem, "DirectMap2M"),
                        direct_map4k: check_whether_key(&mem, "DirectMap4k"),
                        dirty: check_whether_key(&mem, "Dirty"),
                        file_huge_pages: check_whether_key(&mem, "FileHugePages"),
                        file_pmd_mapped: check_whether_key(&mem, "FilePmdMapped"),
                        hardware_corrupted: check_whether_key(&mem, "HardwareCorrupted"),
                        hugepagesize: check_whether_key(&mem, "Hugepagesize"),
                        hugetlb: check_whether_key(&mem, "Hugetlb"),
                        inactive: check_whether_key(&mem, "Inactive"),
                        inactive_anon: check_whether_key(&mem, "Inactive(anon)"),
                        inactive_file: check_whether_key(&mem, "Inactive(file)"),
                        kreclaimable: check_whether_key(&mem, "KReclaimable"),
                        kernel_stack: check_whether_key(&mem, "KernelStack"),
                        mapped: check_whether_key(&mem, "Mapped"),
                        mem_available: check_whether_key(&mem, "MemAvailable"),
                        mem_free: check_whether_key(&mem, "MemFree"),
                        mem_total: check_whether_key(&mem, "MemTotal"),
                        mlocked: check_whether_key(&mem, "Mlocked"),
                        nfs_unstable: check_whether_key(&mem, "NFS_Unstable"),
                        page_tables: check_whether_key(&mem, "PageTables"),
                        percpu: check_whether_key(&mem, "Percpu"),
                        sreclaimable: check_whether_key(&mem, "SReclaimable"),
                        sunreclaim: check_whether_key(&mem, "SUnreclaim"),
                        shmem: check_whether_key(&mem, "Shmem"),
                        shmem_huge_pages: check_whether_key(&mem, "ShmemHugePages"),
                        shmem_pmd_mapped: check_whether_key(&mem, "ShmemPmdMapped"),
                        slab: check_whether_key(&mem, "Slab"),
                        swap_cached: check_whether_key(&mem, "SwapCached"),
                        swap_free: check_whether_key(&mem, "SwapFree"),
                        swap_total: check_whether_key(&mem, "SwapTotal"),
                        unevictable: check_whether_key(&mem, "Unevictable"),
                        vmalloc_chunk: check_whether_key(&mem, "VmallocChunk"),
                        vmalloc_total: check_whether_key(&mem, "VmallocTotal"),
                        vmalloc_used: check_whether_key(&mem, "VmallocUsed"),
                        writeback: check_whether_key(&mem, "Writeback"),
                        writeback_tmp: check_whether_key(&mem, "WritebackTmp"),
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

            let cpu_usage = format!(
                "CPU load: {}% user, {}% nice, {}% system, {}% intr, {}% idle ",
                cpu.user * 100.0,
                cpu.nice * 100.0, // TG :cpu load nice
                cpu.system * 100.0,
                cpu.interrupt * 100.0,
                cpu.idle * 100.0
            );

            return Stats {
                loadavg: load_avg,
                cpu_usage,
                memory_usage: memory_details,
            };
        }
        Err(x) => println!("\nCPU load: error: {}", x),
    }
    Stats {
        loadavg: get_load(0.0, 0.0, 0.0),
        cpu_usage: "Error".to_string(),
        memory_usage: get_empty_memory_usage(),
    }
}

/**
==============================
     HTML Endpoints
==============================
 */

#[get("")]
pub async fn index_page() -> Result<HttpResponse, Error> {
    let sys = System::new();

    match sys.cpu_temp() {
        Ok(cpu_temp) => println!("\nCPU temp: {}", cpu_temp),
        Err(x) => println!("\nCPU temp: {}", x),
    }

    let stats: Stats = get_stats_from_linux(sys).await;
    println!("{:?}", stats);

    let stats_html = StatsTemplate { stats }; // instantiate your struct

    let html_str = minify(&stats_html.render().unwrap());

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html_str))
}

#[get("/v1/stats")]
async fn status_get_api() -> impl Responder {
    let sys = System::new();
    let stats: Stats = get_stats_from_linux(sys).await;
    println!("{:?}", stats);

    let has_error = stats.cpu_usage == "Error";

    let stats_response = StatsResponse {
        result: !has_error,
        data: stats,
    };

    let response = serde_json::to_string(&stats_response).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response)
}

//https://www.anycodings.com/1questions/2410879/using-serdejson-to-serialise-maps-with-non-string-keys

#[cfg(target_os = "linux")]
pub fn check_whether_key(mem: &Memory, key: &str) -> String {
    let d = match mem.platform_memory.meminfo.get(key) {
        Some(active) => active.to_string(),
        _ => "".to_string(),
    };
    return d;
}

/**
==============================
     WebSocket Endpoints
==============================
 */

/// Define HTTP actor
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

#[get("/ws")]
pub async fn ws_stats_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}

/**
==============================
        SSE Endpoints
==============================
 */


