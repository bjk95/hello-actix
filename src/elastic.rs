use elasticsearch::Elasticsearch;
use elasticsearch::cat::CatIndicesParts;
use elasticsearch::SearchParts;
use elasticsearch::Error;
use elasticsearch::http::response::Response;
use elasticsearch::http::transport::Transport;
use crate::configuration::get_config;
use serde_json::json;

fn create_client() -> Elasticsearch {
    let elastic_url = get_config().elastic_url;
    let transport = Transport::single_node(&elastic_url).unwrap(); 
    let client = Elasticsearch::new(transport);
    client
}

pub async fn cat_indices() -> Result<Response, Error> {
    let response = create_client()
        .cat()
        .indices(CatIndicesParts::Index(&["*"]))
        .send()
        .await;

        response
}

pub async fn simple_search(term: &String, number_of_results: &i64, index: &String) -> String {
    let response: Response = create_client()
    .search(SearchParts::Index(&[index]))
    .from(0)
    .size(number_of_results)
    .body(json!({
        "query": {
          "query_string": {
            "query": term
          }
        }
      }))
    .send()
    .await
    .unwrap();

    response.text().await.unwrap()
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
  use super::*;
  
  #[actix_rt::test]
  async fn test_str_len_async() {
      let indices: Response = cat_indices().await.unwrap();
        assert_ne!(indices.text().await.unwrap(), "");
  }
}


