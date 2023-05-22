#[macro_use]
extern crate rocket;

use backend_practice::api::{get_all_routes, MovieRepo, MovieService};
use mongodb::bson::doc;
use std::error::Error;

#[get("/")]
fn index() -> &'static str {
    "Homepage"
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let movie_service: MovieService;

    let _rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/api/v1", get_all_routes())
        .launch()
        .await?;

    Ok(())
}
