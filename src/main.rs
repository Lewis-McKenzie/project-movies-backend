#[macro_use]
extern crate rocket;

use backend_practice::api::{get_all_routes, MovieService, ReviewService};
use mongodb::bson::doc;
use rocket::{fairing::AdHoc, http::Method};
use rocket_cors::{AllowedOrigins, CorsOptions};
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
        .attach(AdHoc::on_ignite("connecting...", |rocket| async {
            rocket.manage(ReviewService::new().await)
        }))
        .attach(
            CorsOptions::default()
                .allowed_origins(AllowedOrigins::all())
                .allowed_methods(
                    vec![Method::Get, Method::Post, Method::Patch]
                        .into_iter()
                        .map(From::from)
                        .collect(),
                )
                .allow_credentials(true)
                .to_cors()?,
        )
        .mount("/", routes![index])
        .mount("/api/v1", get_all_routes())
        .launch()
        .await?;

    Ok(())
}
