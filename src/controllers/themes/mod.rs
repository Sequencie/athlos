/*
 * @description This is a controller module for the sessions resource.
 * @author @0verwtch
 */


/*
 * Core crates. 
 **/
use json::object;
use mongodb::bson::{doc, Document};
use serde_json;
use validator::{HasLen, Validate};
use warp::http::StatusCode;
use warp::hyper::Body;
use warp::reply::Response;
use warp::Rejection;
use serde::Serialize;

/*
 * Custom crates. 
 **/
use crate::helpers::reply::reply_helper;
use crate::models::themes::Theme as ThemeModel;
use crate::services::themes::Theme;

#[derive(Debug, Serialize)]
pub struct ThemeResponse {
    pub message: String,
    pub themes: Option<Vec<ThemeModel>>,
    pub status: u16,
}


impl ThemeResponse {
    fn new(message: String, themes: Option<Vec<ThemeModel>>, status: u16) -> Self {
        Self {
            message,
            themes,
            status,
        }
    }
}

/// Function defining the create action.
pub async fn create(theme: ThemeModel) -> Result<Response, Rejection> {
    let validate_errors = ThemeModel::validate(&theme);
    if let Err(err) = validate_errors {
        let response = reply_helper(
            Body::from(format!("CT0-0: {}", err)),
            StatusCode::UNPROCESSABLE_ENTITY,
        );
        return Ok(response);
    }
    let service: Theme = Theme::new().await;
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
    let re = mongodb::bson::Regex {
        pattern: pattern,
        options: String::from("i"),
    };
    let filter: Document = doc! {"name": re};
    let search_results = service.get(filter).await;
    if search_results.length() == 0 {
        let obj = object! {
            status: 404,
            error: "CT1-0"
        };
        let response = reply_helper(Body::from(obj.to_string()), StatusCode::NOT_FOUND);
        return Ok(response);
    }
    let response: ThemeResponse = ThemeResponse::new("Success".to_string(), Some(search_results), 200);
    let result_string = serde_json::to_string(&response).expect("CT1-1");
    let reply = reply_helper(Body::from(result_string), StatusCode::OK);
    return Ok(reply);
}
