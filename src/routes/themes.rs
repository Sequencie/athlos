/// Core crates.
use warp::Filter;
use serde::{Deserialize};

/// Custom crates.
use crate::controllers::themes::{create, get};
use crate::models::themes::Theme as ThemeModel;

#[derive(Deserialize)]
pub struct SearchQuery{
    theme: String
}
pub fn routes () -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Endpoint for creating themes.
    let create = warp::path!("themes" / "create")
    .and(warp::post())
    .and(warp::body::json())
    .and_then(move |theme: ThemeModel| async {create(theme).await})
    .with(warp::log("api"));

    // Endpoint for getting themes.
    let get = warp::path!("themes" / "get")
    .and(warp::get())
    .and(warp::query().map(|search_query: SearchQuery| search_query.theme))
    .and_then(move |param: String| async {get(param).await})
    .with(warp::log("api"));
    
    return create.or(get);
}