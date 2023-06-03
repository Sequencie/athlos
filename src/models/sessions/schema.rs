use mongodb::bson::oid::ObjectId;
use mongodb::options::UpdateModifications;
use validator::{Validate, ValidationError, ValidationErrors};
use serde::{Serialize, Deserialize, Serializer};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub player_id: String,
    pub _id: Option<ObjectId>,
    pub score: u32,
    pub is_host: bool
}
#[derive(Debug,Deserialize,Serialize, Clone)]
pub struct Session {
    pub _id: Option<ObjectId>,
    pub players: Vec<Player>,
    pub location: String,
    pub is_current: bool
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SessionUpdate{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub players: Option<Vec<PlayerUpdate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_current: Option<bool>
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_host: Option<bool>
}