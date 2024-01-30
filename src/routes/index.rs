use crate::routes::Cache;

use axum::extract::State;

pub async fn index(State(cache): State<Cache>) -> String {

    let count= cache.unindexed_cache.lock().unwrap().len();

    return format!("Roms indexed: {}", count);

}