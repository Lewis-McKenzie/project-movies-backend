use crate::data::Review;

pub struct ReviewRepo {}

impl ReviewRepo {
    pub async fn new() -> ReviewRepo {
        ReviewRepo {}
    }

    pub async fn insert_review(&self, review: Review) -> () {}
}
