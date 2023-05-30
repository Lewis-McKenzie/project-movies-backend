mod movie_controller;
mod movie_repo;
mod movie_service;

use movie_controller::{get_movies, get_movies_by_id};
use movie_repo::MovieRepo;
pub use movie_service::MovieService;

use rocket::Route;

pub fn get_all_routes() -> Vec<Route> {
    routes![get_movies, get_movies_by_id,]
}
