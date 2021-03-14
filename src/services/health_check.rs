use actix_web::{get, Responder};
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

/// Handler to get the liveness of the service
fn get_health() -> HealthResponse {
    HealthResponse {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
    }
}

#[get("/")]
pub async fn health_check() -> impl Responder {
    let res: HealthResponse = get_health();

    HttpResponse::Ok()
        .json(res)
}

