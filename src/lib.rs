
// Route imports
mod routes;
use routes::create_routes;
use crate::routes::game_item::Item;

// MongoDB imports
use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use futures::stream::TryStreamExt;

// Standard library imports
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::error::Error;

// Time imports
use chrono::offset::Utc;
use std::time::Duration;


pub async fn run() -> axum::routing::Router {

    let list= fetch_games().await.unwrap();

    let indexed_cache= Arc::new(Mutex::from(create_indexed_cache(list.clone()).await));

    let unindexed_cache= Arc::new(Mutex::from(create_unindexed_cache(list.clone()).await));

    let app= create_routes(&indexed_cache, &unindexed_cache).await;

    tokio::spawn(update_cache(indexed_cache, unindexed_cache));

    return app

}

async fn fetch_games() -> Result<Vec<Item>, Box<dyn Error>> {

    let uri= "mongodb+srv://LycheeAPI:lycheeengine@cluster0.np31lmg.mongodb.net/?retryWrites=true&w=majority";

    let client_options= ClientOptions::parse_with_resolver_config(uri, ResolverConfig::cloudflare()).await?;
    let client= Client::with_options(client_options)?;

    let collection= client.database("Game_Cache").collection::<Item>("Games");

    let mut cursor= collection.find(None, None).await?;

    let mut list= Vec::new();

    
    while let Some(document)= cursor.try_next().await? {
        
        list.push(document);
        
    }
    
    return Ok(list);

}

async fn create_indexed_cache(list: Vec<Item>) -> HashMap<String, Vec<Item>> {

    let mut cache= HashMap::new();

    let indexes= String::from_utf8(
        (b'a' ..= b'z').collect()
    ).unwrap();

    for index in indexes.chars() {

        cache.insert(index.to_string(), Vec::new());

    }

    cache.insert("other".to_string(), Vec::new());

    for item in list {

        let index= item.name.chars().next().unwrap().to_string();

        match cache.get_mut(&index) {

            Some(items) => items.push(item),

            None => cache.get_mut("other").unwrap().push(item)

        }

    }

    return cache

}

async fn create_unindexed_cache(list: Vec<Item>) -> HashMap<String, Item> {

    let mut cache= HashMap::new();

    for item in list {

        let id= item._id.to_string();

        cache.insert(id, item);

    }

    return cache

}

async fn update_cache(indexed_cache: Arc<Mutex<HashMap<String, Vec<Item>>>>, unindexed_cache: Arc<Mutex<HashMap<String, Item>>>) {

    let mut interval= tokio::time::interval(Duration::from_secs(60));

    loop {

        interval.tick().await;
        
        // checks if utc now is 00:00
        if Utc::now().format("%H").to_string() == "00" {

            let updated_games= fetch_games().await.unwrap();
            
            *indexed_cache.lock().unwrap()= create_indexed_cache(updated_games.clone()).await;
            *unindexed_cache.lock().unwrap()= create_unindexed_cache(updated_games.clone()).await;

            println!("Updated cache at {}", Utc::now().format("%Y-%m-%d %H:%M:%S"));

        }

    }

}