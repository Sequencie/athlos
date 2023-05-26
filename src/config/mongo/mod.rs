use mongodb::{Client, Database, options::ClientOptions};
use dotenv::dotenv;

pub struct MongoConfig{
    client:Client,
}

impl MongoConfig {
    pub async fn new() -> Self {
        dotenv().ok();
        let mongo_uri: String = std::env::var("MONGO_URI").expect("Should've returned a MONGO_URI statement from the database");
        let client_default = Client::with_uri_str(mongo_uri).await.expect("Should have gotten client");
        Self{
            client: client_default,
        }
    }

    pub fn get_client(&self) -> &Client {
        &self.client
    }

    pub fn get_database(&self) -> Database {
        self.client.database("authentication")
    }
}