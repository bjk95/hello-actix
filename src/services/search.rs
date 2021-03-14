
use serde::Deserialize;
use actix_web::{get, web, Result};
use crate::configuration::get_config;
use crate::elastic::simple_search;


#[derive(Deserialize)]
pub struct Search {
    term: String,
    number_of_results: i64
}

impl Default for Search {
    fn default() -> Self {Search{
        term:  "*".into(), 
        number_of_results: 10
    }}
}

#[get("/search/{term}/{number_of_results}")]
pub async fn search_service(search_query: web::Path<Search>) -> Result<String> {
    let i: String = simple_search(&search_query.term, search_query.number_of_results, &"news".into()).await;

    create_search_response(&search_query, &i)
}

fn create_search_response(query: &Search, s: &String) -> Result<String> {
    let config = get_config();
    Ok(format!(
        "searching for {} with {} results from {}, {}", 
        query.term, query.number_of_results, config.elastic_url, s
    )
)
}