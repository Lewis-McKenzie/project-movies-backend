use crate::data::Movie;

use super::MovieRepo;

pub struct MovieService<'a> {
    movie_repo: MovieRepo<'a>,
}

impl<'a> MovieService<'a> {
    pub async fn new() -> MovieService<'a> {
        MovieService {
            movie_repo: MovieRepo::new().await,
        }
    }

    pub async fn get_all_movies(&self) -> Vec<Movie<'a>> {
        let movies: Vec<Movie> = vec![];

        let mut cursor = match self.movie_repo.movie_collection.find(None, None).await {
            Ok(v) => v,
            Err(e) => return movies,
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
}
