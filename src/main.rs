pub mod configuration;
pub mod elastic;
pub mod helpers;
pub mod services;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use env_logger::Env;
use log::info;
use services::health_check::health_check;
use services::search::search_service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("App search started");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(health_check)
            .service(search_service)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
