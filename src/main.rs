#[macro_use]
extern crate rocket;

use backend_practice::api::{get_all_routes, MovieService};
use mongodb::bson::doc;
use rocket::fairing::AdHoc;
use std::error::Error;

#[get("/")]
fn index() -> &'static str {
    "Homepage"
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _rocket = rocket::build()
        .attach(AdHoc::on_ignite("connecting...", |rocket| async {
            rocket.manage(MovieService::new().await)
        }))
        .mount("/", routes![index])
        .mount("/api/v1", get_all_routes())
        .launch()
        .await?;

    Ok(())
}
