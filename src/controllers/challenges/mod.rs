/// Core crates.

use mongodb::bson::{Document, doc};
use warp::Rejection;
use warp::hyper::Body;
use warp::http::StatusCode;
use validator::{HasLen, Validate};
use serde_json;
use warp::reply::Response;
use json::object;

/// Custom crates.
use crate::services::challenges::Challenge;
use crate::helpers::reply::reply_helper;
use crate::models::challenges::Challenge as Model;

/// Create function
pub async fn create(challenge: Model) -> Result<Response, Rejection> {
    // Run validation
    let validate_errors = Model::validate(&challenge);
    if let Err(err) = validate_errors {
        let response = reply_helper(Body::from(format!("CC0-0: {}", err)), StatusCode::UNPROCESSABLE_ENTITY);
        return Ok(response);
    }
    // Initialize services and check for duplicates
    let service:Challenge = Challenge::new().await;
    let filter:Document = doc! {"name": &challenge.name};
    let duplicates = service.get(filter).await;
    if duplicates.length() > 0 {
        let response = reply_helper(Body::from("CC0-1"), StatusCode::BAD_REQUEST);
        return Ok(response);
    }
    // Inserts passed value and returns a Response instance.
    let inserted_result = service.create(challenge).await;
    let response_object = object! {
        created: inserted_result.inserted_id.to_string()
    };
    let response_string = json::stringify(response_object); 
    let reply: Response = reply_helper(Body::from(response_string), StatusCode::CREATED);
    return Ok(reply)
}

/// Function to get all challenges.
pub async fn get() -> Result<Response, Rejection> {
    let filter = doc! {};
    let service:Challenge = Challenge::new().await;
    let search_results: Vec<Model> = service.get(filter).await;
    if search_results.length() == 0 {
        let response = reply_helper(Body::from("CT1-0"), StatusCode::NOT_FOUND);
        return Ok(response);
    }
    let result_string = serde_json::to_string(&search_results).expect("CT1-1");
    let reply = reply_helper(Body::from(result_string), StatusCode::OK);
    return Ok(reply);
}

/// Function to search challenges.
pub async fn search(search_query:String) -> Result<Response, Rejection> {
    let service: Challenge = Challenge::new().await;
    // creating a regular expression;
    let pattern = format!(".*{}*.", search_query);
    let re = mongodb::bson::Regex{
        pattern: pattern,
        options: String::from("i")
    };
    // create filter and return search results.
    let filter: Document = doc!{"name": re};
    let search_results = service.get(filter).await;
    if search_results.length() == 0 {
        let response = reply_helper(Body::from("CT2-0"), StatusCode::NOT_FOUND);
        return Ok(response);
    }
    let result_string = serde_json::to_string(&search_results).expect("CT2-1");
    let reply = reply_helper(Body::from(result_string), StatusCode::OK);
    return Ok(reply);
}