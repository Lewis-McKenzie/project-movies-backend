use bson::doc;
use mongodb::{
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client, Collection,
};
use std::error::Error;

use crate::data::Movie;

pub struct MovieRepo {
    client: Client,
    pub movie_collection: Collection<Movie>,
}

impl MovieRepo {
    pub async fn new() -> MovieRepo {
        let mut client_options = ClientOptions::parse("mongodb+srv://demo-user:pWQjwEPkF2JnBVvk@demo-cluster.tquryab.mongodb.net/?retryWrites=true&w=majority").await.unwrap();

        // Set the server_api field of the client_options object to Stable API version 1
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);

        // Get a handle to the cluster
        let client = Client::with_options(client_options).unwrap();

        let movies = client
            .database("demo-database")
            .collection::<Movie>("demo-collection");

        MovieRepo {
            client: client,
            movie_collection: movies,
        }
    }

    pub async fn ping(&self) -> Result<(), Box<dyn Error>> {
        // Ping the server to see if you can connect to the cluster
        self.client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
            .await?;

        println!("Pinged your deployment. You successfully connected to MongoDB!");
        Ok(())
    }
}
