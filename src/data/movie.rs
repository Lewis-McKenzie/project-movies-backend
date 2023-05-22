use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Movie {
    #[serde(rename = "_id")]
    id: ObjectId,
    #[serde(rename = "imdbId")]
    imdb_id: String,
    title: String,
    #[serde(rename = "releaseDate")]
    release_date: String,
    #[serde(rename = "trailerLink")]
    trailer_link: String,
    genres: Vec<String>,
    poster: String,
    backdrops: Vec<String>,
    #[serde(rename = "reviewIds")]
    review_ids: Vec<String>,
}
