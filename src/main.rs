#[macro_use]
extern crate rocket;

use backend_practice::api::{
    get_all_routes, movie_controller::init_movie_controller, MovieService,
};
use mongodb::bson::doc;
use std::error::Error;

#[get("/")]
fn index() -> &'static str {
    "Homepage"
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_movie_controller();

    let _rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/api/v1", get_all_routes())
        .launch()
        .await?;

    Ok(())
}
