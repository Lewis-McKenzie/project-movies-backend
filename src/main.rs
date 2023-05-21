#[macro_use]
extern crate rocket;

use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};

use std::env;
use std::error::Error;
use tokio;

#[get("/")]
fn index() -> &'static str {
    "Homepage"
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client_options = ClientOptions::parse("mongodb+srv://demo-user:pWQjwEPkF2JnBVvk@demo-cluster.tquryab.mongodb.net/?retryWrites=true&w=majority").await?;
    // Set the server_api field of the client_options object to Stable API version 1
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;
    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");

    let _rocket = rocket::build().mount("/", routes![index]).launch().await?;

    Ok(())
}
