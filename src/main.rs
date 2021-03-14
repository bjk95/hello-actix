pub mod services;
pub mod configuration;
pub mod helpers;
pub mod elastic;

use actix_web::{App, HttpServer};
use services::search::search_service;
use services::health_check::health_check;
use env_logger::Env;
use log::info;


#[actix_web::main]
async fn main() -> std::io::Result<()>{

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("App search started");

    HttpServer::new(|| {
        App::new()
        .service(health_check)
        .service(search_service)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
