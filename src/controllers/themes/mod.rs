/// Core crates
use warp::Rejection;
use warp::reply::Response;
use mongodb::bson::{Document, doc};
use warp::hyper::Body;
use warp::http::StatusCode;
use serde_json;
use validator::{HasLen, Validate};
use json::object;

/// Custom crates
use crate::helpers::reply::reply_helper;
use crate::services::themes::Theme;
use crate::models::themes::Theme as ThemeModel;

/// Function defining the create action.
pub async fn create(theme: ThemeModel) -> Result<Response, Rejection> {
    let validate_errors = ThemeModel::validate(&theme);
    if let Err(err) = validate_errors {
        let response = reply_helper(Body::from(format!("CT0-0: {}", err)), StatusCode::UNPROCESSABLE_ENTITY);
        return Ok(response);
    }
    let service:Theme = Theme::new().await;
    let filter: Document = doc! {"name": &theme.name};
    let duplicates = service.get(filter).await;
    if duplicates.length() > 0 {
        let response = reply_helper(Body::from("CT0-1"), StatusCode::BAD_REQUEST);
        return Ok(response);
    }

    let inserted_result = service.create(theme).await;
    let response_object = object! {
        created: inserted_result.inserted_id.to_string()
    };
    let response_string = json::stringify(response_object);
    let reply: Response = reply_helper(Body::from(response_string), StatusCode::CREATED);
    return Ok(reply);
}

/// Function defining the get action.
pub async fn get(query: String) -> Result<Response, Rejection> {
    let service: Theme = Theme::new().await;
    // Creating a regular expression.
    let pattern = format!(".*{}.*", query);
    let re = mongodb::bson::Regex{
        pattern: pattern,
        options: String::from("i")
    };
    let filter: Document = doc! {"name": re};
    let search_results = service.get(filter).await;
    if search_results.length() == 0 {
        let response = reply_helper(Body::from("CT1-0"), StatusCode::NOT_FOUND);
        return Ok(response);
    }
    let result_string = serde_json::to_string(&search_results).expect("CT1-1");
    let reply = reply_helper(Body::from(result_string), StatusCode::OK);
    return Ok(reply);
}