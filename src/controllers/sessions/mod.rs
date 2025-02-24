/*
 * @description This is a controller module for the sessions resource.
 * @author @0verwtch
 */

/*
 * Core crates. 
 **/
use warp::Rejection;
use bson::oid::ObjectId;
use std::option::Option;
use mongodb::bson::{doc, Document};
use validator::Validate;
use warp::http::StatusCode;
use warp::hyper::Body;
use warp::reply::Response;

/*
 * Custom crates. 
 **/
use crate::helpers::reply::{create_json_response, create_serde_response, reply_helper};
use crate::models::sessions::schema::{Player, PlayerUpdate, RevokeInvite};
use crate::models::sessions::schema::{Session as Model, SessionUpdate};
use crate::services::sessions::Sessions as Service;

/// Create action.
pub async fn create(session: Model) -> Result<Response, Rejection> {
    // Run validation
    let validate_errors = Model::validate(&session);
    if let Err(err) = validate_errors {
        let response = reply_helper(
            Body::from(format!("CS0-0: {}", err)),
            StatusCode::UNPROCESSABLE_ENTITY,
        );
        return Ok(response);
    }

    // Initialize services and isolate the host.
    let service: Service = Service::new().await;
    let host = session
        .players
        .iter()
        .filter(|player| player.is_host == true)
        .collect::<Vec<&Player>>()
        .pop()
        .expect("CS0-0");
    // Check for duplicates with a filter
    let filter: Document =
        doc! {"$and":[{"players.player_id": &host.player_id}, {"is_current": true}]};
    let duplicates = service.get(filter).await;
    if duplicates.len() > 40 { // TODO: Change this back to 0. It is 10 for debugging purposes only
        let response_body =
            create_json_response(true, "A live session was found. Terminate it and try again", None, Option::from("CS0-1"));
        let response = reply_helper(response_body, StatusCode::BAD_REQUEST);
        return Ok(response);
    }
    // Create the session and return a response.
    service.create(session.clone()).await;
    let existing_sessions = service.get(doc!{"players.player_id": &host.player_id}).await;
    let response_data = Some(serde_json::to_string(&existing_sessions).expect("CS0-2"));
    let response_body = create_serde_response(false, "Session created", response_data, None);
    let response = reply_helper(response_body, StatusCode::CREATED);
    return Ok(response);
}

/// Get action.
pub async fn get(player_id: String, is_current: bool) -> Result<Response, Rejection> {
    // create a service.
    let service: Service = Service::new().await;
    let filter: Document =
        doc! { "$and": [{"players.player_id": player_id}, {"is_current": is_current}] };
    let results = service.get(filter).await;
    if results.is_empty() {
        let response_body =
            create_json_response(true, "No such session", None, Option::from("CS1-0"));
        return Ok(reply_helper(response_body, StatusCode::NOT_FOUND));
    }
    // Return a response.
    let response_data = Option::from(serde_json::to_string(&results).expect("CS1-1"));
    let response_body = create_serde_response(false, "Sessions obtained", response_data, None);
    let response = reply_helper(response_body, StatusCode::OK);
    return Ok(response);
}

/// Update action
pub async fn update(session_id: String, updates: SessionUpdate) -> Result<Response, Rejection> {
    // Create a service.
    let service: Service = Service::new().await;
    let filter: Document = doc! { "_id": ObjectId::parse_str(session_id).expect("CS2-0") };
    let update_document = mongodb::bson::to_document(&updates).expect("CS2-1");
    let modified_update_document = doc! {"$set": update_document};
    let update_result = service.update(filter, modified_update_document).await;
    // Commit the update.
    let response_data = Option::from(serde_json::to_string(&update_result).unwrap());
    if update_result.modified_count == 0 {
        let response_body =
            create_json_response(true, "Update didn't work", None, Option::from("CS2-2"));
        return Ok(reply_helper(response_body, StatusCode::BAD_REQUEST));
    }
    // Return a response.
    let response_body = create_serde_response(false, "Update was a success", response_data, None);
    let response = reply_helper(response_body, StatusCode::OK);
    return Ok(response);
}

/// Invite action.
pub async fn invite(session_id: String, mut updates: PlayerUpdate) -> Result<Response, Rejection> {
    // Create a service.
    let service: Service = Service::new().await;

    // Create a new ObjectId to give the new player.
    let object_id = ObjectId::new();
    updates._id = Option::from(object_id);

    // Define the filter.
    let filter: Document = doc! { "_id": ObjectId::parse_str(session_id).expect("CS3-0") };
    let update_document = mongodb::bson::to_document(&updates).expect("CS3-1");
    let modified_update_document = doc! {"$push": {"players": update_document}};
    println!("{}", modified_update_document);
    let update_result = service.update(filter, modified_update_document).await;

    // Commit the update.
    let response_data = Option::from(serde_json::to_string(&update_result).unwrap());
    if update_result.modified_count == 0 {
        let response_body =
            create_json_response(true, "Update didn't work", None, Option::from("CS3-3"));
        return Ok(reply_helper(response_body, StatusCode::BAD_REQUEST));
    }

    // Return a response.
    let response_body = create_serde_response(false, "Update was a success", response_data, None);
    let response = reply_helper(response_body, StatusCode::OK);
    return Ok(response);
}

/// Revoke action.
pub async fn revoke(session_id: String, updates: RevokeInvite) -> Result<Response, Rejection> {
    // Create a service.
    let service: Service = Service::new().await;

    // Create the filter and update body.
    let filter: Document = doc! { "_id": ObjectId::parse_str(session_id).expect("CS3-0") };
    let modified_update_document = doc! {"$pull": { "players": {"player_id": updates.player_id } }};
    println!("{}", modified_update_document);
    let update_result = service.update(filter, modified_update_document).await;

    // Commit the update.
    let response_data = Option::from(serde_json::to_string(&update_result).unwrap());
    if update_result.modified_count == 0 {
        let response_body =
            create_json_response(true, "Update didn't work", None, Option::from("CS3-3"));
        return Ok(reply_helper(response_body, StatusCode::BAD_REQUEST));
    }

    // Return a response.
    let response_body = create_serde_response(false, "Update was a success", response_data, None);
    let response = reply_helper(response_body, StatusCode::OK);
    return Ok(response);
}
