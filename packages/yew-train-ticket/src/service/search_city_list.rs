#![allow(unused_variables)]
use super::fetch::Fetch;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchCityResult {
    pub result: Vec<SearchResult>,
    pub searchKey: String,
}

impl SearchCityResult {
    pub fn new() -> SearchCityResult {
        SearchCityResult {
            result: vec![],
            searchKey: "".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub key: String,
    pub display: String,
}

// #[wasm_bindgen]
pub async fn search_city_list(search_word: String) -> SearchCityResult {
    let url = format!("http://localhost:80/rest/search?key={}", search_word);
    let json = Fetch::get(url).await;

    match json {
        Ok(json) => json.into_serde().unwrap(),
        Err(_) => SearchCityResult::new(),
    }
}
