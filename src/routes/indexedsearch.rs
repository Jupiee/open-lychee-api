
use crate::routes::{Cache, game_item::Item, remove_special_characters, check_items, check_items_platform};

use axum::extract::{State, Query};

use serde::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {

    name: String,
    limit: Option<i64>,
    platform: String

}

pub async fn indexed_search(Query(params): Query<SearchQuery>, State(cache): State<Cache>) -> String {

    let indexed_cache= cache.indexed_cache.lock().unwrap();

    let lower_cased_name= params.name.to_lowercase();

    let initial= &lower_cased_name.chars().nth(0).unwrap().to_string();
    
    let query_words= lower_cased_name.split_whitespace().map(|word| remove_special_characters(word)).collect::<String>();

    let matching_items: Vec<Item>;

    if params.platform == "All" {

        match indexed_cache.get(initial) {

            Some(items) => {

                matching_items= check_items(items, query_words);

            },

            None => {

                let other_items= indexed_cache.get("other").unwrap();

                matching_items= check_items(other_items, query_words);

            }

        }

    }

    else {

        let platform: Vec<&str>= params.platform.split(",").collect();

        match indexed_cache.get(initial) {

            Some(items) => {

                matching_items= check_items_platform(items, query_words, platform);

            },

            None => {

                let other_items= indexed_cache.get("other").unwrap();

                matching_items= check_items_platform(other_items, query_words, platform);

            }

        }

    }

    match params.limit {
    
        Some(limit) => return serde_json::to_string(&matching_items[..limit as usize]).unwrap(),
        None => return serde_json::to_string(&matching_items).unwrap()

    }

}