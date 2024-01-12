use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Item {

    pub _id: ObjectId,
    pub name: String,
    file_size: String,
    pub platforms: String,
    download_links: String,

}