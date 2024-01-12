
use crate::routes::game_item::Item;

use axum::extract::{State, Query};

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use serde::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {

    name: String,
    limit: Option<i64>,
    platform: String

}

fn remove_special_characters(word: &str) -> String {

    word.chars().filter(|&character| !character.is_ascii_punctuation()).collect()

}

pub async fn search(Query(params): Query<SearchQuery>, State(cache): State<Arc<Mutex<HashMap<String, Item>>>>) -> String {

    let cache= cache.lock().unwrap();

    
    let lower_cased_name= params.name.to_lowercase();
    
    let query_words= lower_cased_name.split_whitespace().map(|word| remove_special_characters(word)).collect::<String>();
    
    let mut matching_items = Vec::new();
    
    if params.platform == "All" {
        
        for item in cache.values() {
    
            let lower_cased_item_name= item.name.to_lowercase();
    
            let item_name_words = lower_cased_item_name.split_whitespace().map(|word| remove_special_characters(word)).collect::<String>();
    
            if item_name_words.contains(&query_words) {
    
                matching_items.push(item.clone());
    
            }
            
        }
    
        match params.limit {
    
            Some(limit) => return serde_json::to_string(&matching_items[..limit as usize]).unwrap(),
            None => return serde_json::to_string(&matching_items).unwrap()
    
        }
        
    }
    
    else {

        let platform: Vec<&str>= params.platform.split(",").collect();
        
        for item in cache.values() {
    
            let lower_cased_item_name= item.name.to_lowercase();
    
            let item_name_words = lower_cased_item_name.split_whitespace().map(|word| remove_special_characters(word)).collect::<String>();
    
            if item_name_words.contains(&query_words) && platform.contains(&item.platforms.as_str()) {
    
                matching_items.push(item.clone());
    
            }
            
        }
    
        match params.limit {
    
            Some(limit) => return serde_json::to_string(&matching_items[..limit as usize]).unwrap(),
            None => return serde_json::to_string(&matching_items).unwrap()
    
        }

    }

}