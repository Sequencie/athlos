/*
 * @description This is a model module for the sessions resource. 
 * All models needed for sessions are defined and managed here
 * @author @0verwtch
 */

/*
 * Core crates. 
 **/
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Approval {
    Requested,
    Denied,
    Approved
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub player_id: ObjectId,
    pub _id: Option<ObjectId>,
    pub score: u32,
    pub is_host: bool,
}

#[derive(Debug,Deserialize,Serialize, Clone)]
pub struct Session {
    pub _id: Option<ObjectId>,
    pub players: Vec<Player>,
    pub location: ObjectId,
    pub is_current: Option<bool>,
    pub approval: Option<Approval>,
    pub theme_id: ObjectId
}




#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SessionUpdate{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub players: Option<Vec<PlayerUpdate>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ObjectId>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_current: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval: Option<Approval>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_id: Option<ObjectId>
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RevokeInvite {
    pub player_id: String
}
