use crate::configuration::get_config;
use crate::elastic::simple_search;
use actix_web::{get, web, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Search {
    term: String,
    results: Option<i64>,
}

#[get("/search")]
pub async fn search_service(search_query: web::Query<Search>) -> Result<String> {
    let i: String = simple_search(&search_query.term, search_query.results, &"news".into()).await;

    create_search_response(&search_query, &i)
}

fn create_search_response(query: &Search, s: &String) -> Result<String> {
    let config = get_config();
    Ok(format!(
        "searching for {} from {}: {}",
        query.term, config.elastic_url, s
    ))
}
