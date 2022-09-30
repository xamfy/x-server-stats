use actix_web::http::StatusCode;
use actix_web::{get, post, web, Error, HttpResponse};
use askama::Template;
use deadpool_postgres::{Client, Pool};
use minify::html::minify;

use crate::Stats;

extern crate minify;

extern crate systemstat;
use std::thread;
use std::time::Duration;
use systemstat::{saturating_sub_bytes, Platform, System};
use systemstat::platform::PlatformImpl;

#[derive(Template)] // this will generate the code...
#[template(path = "index.html")] // using the template in this path, relative
                                 // to the `templates` dir in the crate root
struct StatsTemplate {
    stats: Stats,
}

pub async fn get_stats_from_linux(sys: PlatformImpl) -> Stats {
    match sys.cpu_load_aggregate() {
        Ok(cpu) => {
            println!("\nMeasuring CPU load...");
            thread::sleep(Duration::from_secs(1));
            let cpu = cpu.done().unwrap();
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

            let stats = Stats {
                loadavg: "1.0".to_string(),
                cpu_usage,
                memory_usage: "3.0".to_string(),
            };

            return stats;
        }
        Err(x) => println!("\nCPU load: error: {}", x),
    }

    let stats = Stats {
        loadavg: "Error".to_string(),
        cpu_usage: "Error".to_string(),
        memory_usage: "Error".to_string(),
    };

    return stats;
}

#[get("")]
pub async fn index_page(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    // parameter for this method - req: &HttpRequest
    // println!("{:?}", req);

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
