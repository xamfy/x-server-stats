#![deny(clippy::all)]

mod config;
mod handlers;
mod stats;

use std::env;

use ::config::Config;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_lab::web as web_lab;
use dotenv::dotenv;
use handlers::index_page;
use tokio_postgres::NoTls;

use crate::config::ServerConfig;
use crate::handlers::status_get_api;

use stats::Stats;

fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(index_page);
}

fn api_scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(status_get_api);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    dotenv().ok();

    let config_ = Config::builder()
        .add_source(::config::Environment::default().separator("__"))
        .build()
        .unwrap();

    let config: ServerConfig = config_.try_deserialize().unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    // Allow bursts with up to 10 requests per IP address
    // and replenishes one element every two seconds
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(10)
        .finish()
        .unwrap();

    let base_addr = env::var("BASE_ADDR").expect("BASE_ADDR not set"); // BASE_ADDR could be replaced with ServerConfig.server_addr

    let server = HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            // Enable Governor middleware
            .wrap(Governor::new(&governor_conf))
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/api").configure(api_scoped_config))
            .service(web::scope("/stats").configure(scoped_config))
            .service(web_lab::Redirect::new(
                "/",
                format!("{}{}", base_addr, "/stats"),
            ))
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}
