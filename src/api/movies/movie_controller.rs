use mongodb::bson::oid::ObjectId;
use rocket::{response::status::BadRequest, serde::json::Json, State};

use crate::data::Movie;

use super::MovieService;

#[get("/movies")]
pub async fn get_movies(movie_service: &State<MovieService>) -> Json<Vec<Movie>> {
    Json(movie_service.get_all_movies().await)
}

#[get("/movies/<id>")]
pub async fn get_movies_by_id(
    movie_service: &State<MovieService>,
    id: String,
) -> Result<Json<Movie>, BadRequest<Json<String>>> {
    let oid = match ObjectId::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return Err(BadRequest(Some(Json(format!("Invalid id: {}", &id))))),
    };

    match movie_service.get_movie_by_id(oid).await {
        Ok(movie_option) => match movie_option {
            Some(movie) => Ok(Json(movie)),
            None => Err(BadRequest(Some(Json(format!("No movie with id {}", &id))))),
        },
        Err(e) => Err(BadRequest(Some(Json(e.to_string())))),
    }
}
