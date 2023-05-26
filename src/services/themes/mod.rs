// Core services.

use mongodb::{Collection, Cursor};
use futures::stream::TryStreamExt;
use mongodb::options::FindOptions;
use mongodb::bson::{Document};
use mongodb::results::InsertOneResult;

// Custom crates.
use crate::config::mongo;
use crate::models::themes::Theme as ThemeModel;

/// Base structure.
pub struct Theme {
    collection: Collection<ThemeModel>
}

impl Theme {

    /// Constructor for initializing databases.
    pub async fn new() -> Self {
        let mongo_config = mongo::MongoConfig::new().await;
        let database = mongo_config.get_database();
        Self {collection: database.collection("themes")}
    }

    /// Function to create a theme. 
    pub async fn create(&self, mut theme: ThemeModel) -> InsertOneResult {
        let object_id = Option::from(mongodb::bson::oid::ObjectId::new());
        theme._id = object_id;
        let insert_result: InsertOneResult = self.collection.insert_one(theme, None).await.expect("ST1-0");
        return insert_result;
    }

    /// Function to pull themes using a filter.
    pub async fn get(&self, filter: Document) -> Vec<ThemeModel> {
        let options: FindOptions = FindOptions::builder().build();
        let cursor: Cursor<ThemeModel> = self.collection.find(filter, options).await.expect("ST2-0");
        let results: Vec<ThemeModel> = cursor.try_collect().await.expect("ST2-1");
        return results;
    }
}