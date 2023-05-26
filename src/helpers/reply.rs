// Core crates.
use warp::{http::Response};
use warp::http::StatusCode;
use warp::http::header;
use warp::hyper::Body;

/// A reply helper used to generate replies for each controller and middleware.
pub fn reply_helper(body:Body, status_code: StatusCode) -> Response<Body>  {
    let response = Response::builder()
        .status(status_code)
        .header(header::CONTENT_TYPE, "application/json")
        .body(body)
        .unwrap();
    return response;
}