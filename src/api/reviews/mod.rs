mod review_controller;
mod review_repo;
mod review_service;

use review_controller::create_review;
use review_repo::ReviewRepo;
pub use review_service::ReviewService;
use rocket::Route;

pub fn get_all_routes() -> Vec<Route> {
    routes![create_review]
}
