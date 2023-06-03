/// Core services.
use mongodb::{Collection, Cursor};
use futures::stream::TryStreamExt;
use mongodb::options::FindOptions;
use mongodb::bson::{Document};
use mongodb::results::InsertOneResult;

/// Custom crates.
use crate::models::challenges::Challenge as Model;
use crate::config::mongo;

/// Base structure.
pub struct Challenge {
    collection: Collection<Model>,
}

impl Challenge {
    // Constructor for initializing databases.
    pub async fn new() -> Self {
        let mongo_config = mongo::MongoConfig::new().await;
        let database = mongo_config.get_database();
        Self {collection: database.collection("challenges")}
    }

    // Function to create a challenge.
    pub async fn create(&self, mut challenge: Model) -> InsertOneResult {
        let object_id = Option::from(mongodb::bson::oid::ObjectId::new());
        challenge._id = object_id;
        let insert_result: InsertOneResult = self.collection.insert_one(challenge, None).await.expect("SC0-0");
        return insert_result;
    }

    // Function to get challenges by filter.
    pub async fn get(&self, filter: Document) -> Vec<Model> {
        let find_options: FindOptions = FindOptions::builder().build();
        let cursor: Cursor<Model> = self.collection.find(filter, find_options).await.expect("SC0-1");
        let results: Vec<Model> = cursor.try_collect().await.expect("ST0-2");
        return results;
    }


}