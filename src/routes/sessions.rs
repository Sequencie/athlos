// Core crates.
use warp::Filter;

// Custom crates.
use crate::controllers::sessions;
use crate::controllers::sessions::QueryParams;
use crate::models::sessions::schema::{Session, SessionUpdate};

pub fn routes () -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Create endpoint
    let create = warp::path!("sessions" / "add")
                 .and(warp::post())
                 .and(warp::body::json())
                 .and_then(move |session: Session| async {sessions::create(session).await})
                 .with(warp::log("api"));
    // Get endpoint
    let get = warp::path!("sessions" / "get" / "player")
                .and(warp::get())
                .and(warp::query().map(move |search_query: QueryParams| search_query))
                .and_then(move |query_params: QueryParams| async { sessions::get(query_params).await })
                .with(warp::log("api"));
    // Update endpoint
    let update = warp::path!("sessions" / "update" / "session" / String)
        .and(warp::patch())
        .and(warp::body::json())
        .and_then(move |session_id: String, session: SessionUpdate| async {sessions::update(session_id, session).await})
        .with(warp::log("api"));
    
    return create
            .or(get)
            .or(update);
}   