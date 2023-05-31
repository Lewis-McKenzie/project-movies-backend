use bson::oid::ObjectId;

use crate::data::Review;

use super::ReviewRepo;

pub struct ReviewService {
    review_repo: ReviewRepo,
}

impl ReviewService {
    pub async fn new() -> ReviewService {
        ReviewService {
            review_repo: ReviewRepo::new().await,
        }
    }

    pub async fn create_review(&self, body: String, movie_id: String) -> () {
        self.review_repo.insert_review(Review::new(body)).await;
    }
}
