/*
 * @description This is a helper module for creating replies in needed formats.
 * @author @0verwtch
 */

/*
 * Core crates.
 **/
use json::stringify;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json, to_string};
use warp::http::header;
use warp::http::Response;
use warp::http::StatusCode;
use warp::hyper::Body;

/// Custom types.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct SerdeJsonResponse {
    status: String,
    message: String,
    data: Option<String>,
    error_code: Option<String>,
}

/// A reply helper used to generate replies for each controller and middleware.
pub fn reply_helper(body: Body, status_code: StatusCode) -> Response<Body> {
    let response = Response::builder()
        .status(status_code)
        .header(header::CONTENT_TYPE, "application/json")
        .body(body)
        .unwrap();
    return response;
}

/// A Helper to create json response objects;
pub fn create_json_response(
    is_error: bool,
    message: &str,
    data: Option<Value>,
    error_code: Option<&str>,
) -> Body {
    let response_object = if is_error == true {
        json! ({"status": "error", "message": message, "error_code": error_code, })
    } else {
        json! ({"status": "success", "message": message, "data": data})
    };
    let response_string = stringify(response_object.to_string());
    return Body::from(response_string);
}

/// A helper to create json response objects with serde_json
pub fn create_serde_response(
    is_error: bool,
    message: &str,
    data: Option<String>,
    error_code: Option<String>,
) -> Body {
    let response_object;
    if is_error {
        response_object = SerdeJsonResponse {
            status: "error".to_string(),
            message: message.to_string(),
            data: None,
            error_code: error_code,
        };
    } else {
        response_object = SerdeJsonResponse {
            status: "success".to_string(),
            message: message.to_string(),
            data: data,
            error_code: None,
        };
    };
    let response_string = to_string(&response_object).expect("HR1-0");
    return Body::from(response_string);
}
