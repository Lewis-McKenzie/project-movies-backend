use rocket::serde::json::Json;

use crate::data::Movie;

use super::MovieService;

pub struct MovieController {
    movie_service: Option<MovieService>,
}

impl MovieController {
    pub async fn new() -> MovieController {
        MovieController {
            movie_service: Some(MovieService::new().await),
        }
    }

    pub async fn get_all_movies(&self) -> Vec<Movie> {
        self.movie_service.as_ref().unwrap().get_all_movies().await
    }
}

static movie_controller: MovieController = MovieController {
    movie_service: None,
};

pub async fn init_movie_controller() -> () {
    movie_controller = MovieController::new().await;
}

#[get("/movies")]
pub async fn get_movies() -> Json<Vec<Movie>> {
    Json(movie_controller.get_all_movies().await)
}
