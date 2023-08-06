use rocket::{response::status::BadRequest, serde::json::Json, State};

use crate::api::ReviewService;
use crate::data::ReviewInput;

#[post("/reviews", data = "<payload>")]
pub async fn create_review(
    review_service: &State<ReviewService>,
    payload: Json<ReviewInput>,
) -> () {
    review_service
        .create_review(payload.body.clone(), payload.movie_id.clone())
        .await
}
