#[macro_use]
extern crate rocket;

use backend_practice::{
    api::{get_all_routes, MovieRepo},
    data::Movie,
};
use bson::{Bson, Document};
use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client, Collection,
};
use rocket::futures::StreamExt;
use std::error::Error;

#[get("/")]
fn index() -> &'static str {
    "Homepage"
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let movie_repo = MovieRepo::new();
    let mut cursor = movie_repo.find(None, None).await?;

    while cursor.advance().await? {
        let movie = cursor.deserialize_current()?;
        println!("{:?}", movie);
    }

    let _rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/api/v1", get_all_routes())
        .launch()
        .await?;

    Ok(())
}
