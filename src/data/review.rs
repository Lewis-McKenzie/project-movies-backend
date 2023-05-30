use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Review {
    #[serde(rename = "_id")]
    id: ObjectId,
    body: String,
}
