
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

    let cache= Arc::new(Mutex::from(fetch_games().await.unwrap()));

    let app= create_routes(&cache).await;

    tokio::spawn(update_cache(cache));

    return app

}

async fn fetch_games() -> Result<HashMap<String, Item>, Box<dyn Error>> {

    let uri= "Your MongoDB URI";

    let client_options= ClientOptions::parse_with_resolver_config(uri, ResolverConfig::cloudflare()).await?;
    let client= Client::with_options(client_options)?;

    let collection= client.database("Game_Cache").collection::<Item>("Games");

    let mut cursor= collection.find(None, None).await?;

    let mut cache= HashMap::new();

    while let Some(document)= cursor.try_next().await.unwrap() {

        let id= document._id.to_string();

        cache.insert(id, document);

    }

    return Ok(cache);

}

async fn update_cache(cache: Arc<Mutex<HashMap<String, Item>>>) {

    let mut interval= tokio::time::interval(Duration::from_secs(60));

    loop {

        interval.tick().await;
        
        // checks if utc now is 00:00
        if Utc::now().format("%H").to_string() == "00" {

            let updated_games= fetch_games().await.unwrap();

            *cache.lock().unwrap()= updated_games;

            println!("Updated cache at {}", Utc::now().format("%Y-%m-%d %H:%M:%S"));

        }

    }

}