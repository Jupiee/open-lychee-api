
// Handlers imports
mod index;
mod search;
use search::search;
use index::index;

// Game item import
pub mod game_item;
use game_item::Item;

// Axum imports
use axum::{Router, routing::get};

// Tower imports
use tower_http::cors::CorsLayer;

// HTTP imports
use http::Method;

// Standard library imports
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;


pub async fn create_routes(cache: &Arc<Mutex<HashMap<String, Item>>>) -> Router {

    let routes= Router::new()
        .route("/", get(index))
        .route("/search", get(search))
        .with_state(cache.clone())
        .layer(
            CorsLayer::new()
                .allow_methods(vec![Method::GET, Method::POST])
                .max_age(Duration::from_secs(60 * 60 * 24))
        );

    return routes;

}