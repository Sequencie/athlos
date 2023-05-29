/// Custom crates

use crate::models::challenges::Challenge;
use crate::controllers::challenges;

/// Core crates.
use warp::Filter;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchQuery {
    challenge: String
}

pub fn routes () -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    // Endpoint definition for creating challenges.
    let create = warp::path!("challenges" / "create")
    .and(warp::post())
    .and(warp::body::json())
    .and_then(move |challenge: Challenge| async {challenges::create(challenge).await})
    .with(warp::log("api"));
    // Endpoints defintions for getting all challenges.
    let get = warp::path!("challenges" / "get" / "all")
    .and(warp::get())
    .and_then(move || async {challenges::get().await})
    .with(warp::log("api"));
    // Endpoints definitions for getting challenges by search
    let search = warp::path!("challenges" / "get")
    .and(warp::get())
    .and(warp::query().map(move |search_query: SearchQuery| search_query.challenge))
    .and_then(move |param: String| async {challenges::search(param).await})
    .with(warp::log("api"));
    return create.or(get).or(search);
}