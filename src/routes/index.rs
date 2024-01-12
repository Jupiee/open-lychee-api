use crate::routes::game_item::Item;

use axum::extract::State;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub async fn index(State(cache): State<Arc<Mutex<HashMap<String, Item>>>>) -> String {

    let count= cache.lock().unwrap().len();

    return format!("Roms indexed: {}", count);

}