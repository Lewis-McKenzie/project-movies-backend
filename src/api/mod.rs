mod movies;
mod reviews;

pub use movies::MovieService;
pub use reviews::ReviewService;
use rocket::Route;

pub fn get_all_routes() -> Vec<Route> {
    movies::get_all_routes()
        .iter()
        .chain(reviews::get_all_routes().iter())
        .cloned()
        .collect()
}
