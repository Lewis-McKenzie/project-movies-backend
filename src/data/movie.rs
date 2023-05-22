use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Movie<'doc> {
    #[serde(rename = "_id")]
    id: ObjectId,
    #[serde(rename = "imdbId", borrow)]
    imdb_id: &'doc str,
    #[serde(borrow)]
    title: &'doc str,
    #[serde(rename = "releaseDate", borrow)]
    release_date: &'doc str,
    #[serde(rename = "trailerLink", borrow)]
    trailer_link: &'doc str,
    #[serde(borrow)]
    genres: Vec<&'doc str>,
    #[serde(borrow)]
    poster: &'doc str,
    #[serde(borrow)]
    backdrops: Vec<&'doc str>,
    #[serde(rename = "reviewIds", borrow)]
    review_ids: Vec<&'doc str>,
}
