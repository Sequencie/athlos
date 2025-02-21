/*
 * @description This is a rounte module for the sessions resource.
 * @author @0verwtch
 **/

/*
 * Core crates. 
 **/
use warp::Filter;

/*
 * Custom crates. 
 **/
use crate::controllers::sessions;
use crate::models::sessions::schema::{Session, SessionUpdate, PlayerUpdate, RevokeInvite};

pub fn routes () -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Create endpoint
    let create = warp::path!("sessions" / "add")
                 .and(warp::post())
                 .and(warp::body::json())
                 .and_then(move |session: Session| async {sessions::create(session).await})
                 .with(warp::log("api"));
    // Get endpoint
    let get = warp::path!("sessions" / "get" / "player" / String / bool)
                .and(warp::get())
                .and_then(move |player_id: String, is_current: bool| async move { sessions::get(player_id, is_current).await })
                .with(warp::log("api"));
    // Update endpoint
    let update = warp::path!("sessions" / "update" / "session" / String)
        .and(warp::patch())
        .and(warp::body::json())
        .and_then(move |session_id: String, session: SessionUpdate| async {sessions::update(session_id, session).await})
        .with(warp::log("api"));
    
    // Invite endpoint.
    let invite = warp::path!("sessions" / "invite" / "session" / String)
    .and(warp::patch())
    .and(warp::body::json())
    .and_then(move |session_id: String, player: PlayerUpdate| async {sessions::invite(session_id, player).await})
    .with(warp::log("api"));

    // Revoke endpoint.
    let revoke = warp::path!("sessions" / "revoke" / "session" / String)
    .and(warp::patch())
    .and(warp::body::json())
    .and_then(move |session_id: String, revoke: RevokeInvite| async {sessions::revoke(session_id, revoke).await})
    .with(warp::log("api"));
    return create
            .or(get)
            .or(update)
            .or(invite)
            .or(revoke);
}   