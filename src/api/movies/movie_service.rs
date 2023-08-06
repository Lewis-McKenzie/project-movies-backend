use bson::oid::ObjectId;

use crate::data::Movie;

use super::MovieRepo;

pub struct MovieService {
    movie_repo: MovieRepo,
}

impl MovieService {
    pub async fn new() -> MovieService {
        MovieService {
            movie_repo: MovieRepo::new().await,
        }
    }

    pub async fn get_all_movies(&self) -> Vec<Movie> {
        let mut movies: Vec<Movie> = vec![];

        let mut cursor = match self.movie_repo.find_all_movies().await {
            Ok(v) => v,
            Err(_) => return movies,
        };

        while cursor.advance().await.unwrap_or(false) {
            let movie = match cursor.deserialize_current() {
                Ok(v) => v,
                Err(_) => return movies,
            };
            movies.push(movie);
        }
        movies
    }

    pub async fn get_movie_by_id(
        &self,
        id: &String,
    ) -> Result<Option<Movie>, mongodb::error::Error> {
        self.movie_repo.find_movie_by_id(id).await
    }
}
