
// Handlers imports
mod index;
mod linearsearch;
mod indexedsearch;
use indexedsearch::indexed_search;
use linearsearch::linear_search;
use index::index;

// Game item import
pub mod game_item;
use game_item::Item;

// Axum imports
use axum::{Router, routing::get}; //use axum::{Router, middleware, routing::get};

// Tower imports
use tower_http::cors::CorsLayer;

// HTTP imports
use http::Method;

// Standard library imports
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Cache {

    pub indexed_cache: Arc<Mutex<HashMap<String, Vec<Item>>>>,
    pub unindexed_cache: Arc<Mutex<HashMap<String, Item>>>

}

fn remove_special_characters(word: &str) -> String {

    word.chars().filter(|&character| !character.is_ascii_punctuation()).collect()

}

fn check_items(items: &Vec<Item>, query: String) -> Vec<Item> {

    let mut matching_items = Vec::new();

    for item in items {


        let lower_cased_item_name= item.name.to_lowercase();

        let item_name_words = lower_cased_item_name.split_whitespace().map(|word| remove_special_characters(word)).collect::<String>();

        if item_name_words.contains(&query) {

            matching_items.push(item.clone());

        }

    }

    return matching_items;

}

fn check_items_platform(items: &Vec<Item>, query: String, platform: Vec<&str>) -> Vec<Item> {

    let mut matching_items = Vec::new();

    for item in items {

        let lower_cased_item_name= item.name.to_lowercase();

        let item_name_words = lower_cased_item_name.split_whitespace().map(|word| remove_special_characters(word)).collect::<String>();

        if item_name_words.contains(&query) && platform.contains(&item.platforms.as_str()) {

            matching_items.push(item.clone());

        }

    }

    return matching_items;

}

pub async fn create_routes(indexed_cache: &Arc<Mutex<HashMap<String, Vec<Item>>>>, unindexed_cache: &Arc<Mutex<HashMap<String, Item>>>) -> Router {

    let state= Cache {

        indexed_cache: indexed_cache.clone(),
        unindexed_cache: unindexed_cache.clone()

    };

    let routes= Router::new()
        .route("/", get(index))
        .route("/linearsearch", get(linear_search))
        .route("/indexedsearch", get(indexed_search))
        .with_state(state)
        .layer(
            CorsLayer::new()
                .allow_methods(vec![Method::GET, Method::POST])
                .max_age(Duration::from_secs(60 * 60 * 24))
        );

    return routes;

}