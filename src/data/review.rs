use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Review {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub body: String,
}

impl Review {
    pub fn new(body: String) -> Review {
        Review {
            body: body,
            id: ObjectId::new(),
        }
    }
}
