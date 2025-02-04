/// Core crates.
use mongodb::{Collection, Cursor, options::UpdateOptions};
use futures::stream::TryStreamExt;
use mongodb::options::FindOptions;
use mongodb::bson::Document;
use mongodb::results::{InsertOneResult, UpdateResult};
use mongodb::bson::oid::ObjectId;

/// Custom crates.
use crate::models::sessions::schema::Session as SessionModel;
use crate::config::mongo;

pub struct Sessions {
    collection: Collection<SessionModel>,
}

impl Sessions {
    /// Constructor for initializing databases.
    /// Self {} initializes an instance that is returned by the constructor
    pub async fn new() -> Self {
        let mongo_config = mongo::MongoConfig::new().await;
        let database = mongo_config.get_database();
        Self {collection: database.collection("sessions")}
    }
    
    /// Function to create a session.
    pub async fn create(&self, mut session: SessionModel) -> InsertOneResult {
        let object_id = Option::from(ObjectId::new());
        for player in  &mut session.players {
            player._id = Option::from(ObjectId::new());
        }
        session._id = object_id;
        session.is_current = true;
        let insert_result: InsertOneResult = self.collection.insert_one(session, None).await.expect("SS0-0");
        return insert_result;
    }
    
    /// Function to pull session.
    pub async fn get(&self, filter: Document) -> Vec<SessionModel> {
        let options: FindOptions = FindOptions::builder().build();
        let cursor: Cursor<SessionModel> = self.collection.find(filter, options).await.expect("SS2-0");
        let results: Vec<SessionModel> = cursor.try_collect().await.expect("SS2-1");
        return results;
    }
    /// Function to update
    pub async fn update(&self, query: Document, updates: Document) -> UpdateResult {
        let results = self.collection.update_one(query, updates,  UpdateOptions::default())
            .await
            .expect("SS3-0");
        return results;
    }
}