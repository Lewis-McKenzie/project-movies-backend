pub mod movie_controller;
pub mod movie_repo;
pub mod movie_service;

pub use movie_controller::{get_movies, get_movies_by_id};
pub use movie_repo::MovieRepo;
pub use movie_service::MovieService;
use rocket::Route;

pub fn get_all_routes() -> Vec<Route> {
    routes![get_movies, get_movies_by_id,]
}
