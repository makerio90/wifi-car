use super::Sessions;
use hyper::body::Body;
use log::debug;
use std::convert::Infallible;
use warp::http::{Response, StatusCode};

pub async fn login(sessions: Sessions, id: String) -> Result<impl warp::Reply, Infallible> {
    sessions.lock().unwrap().push(id.clone());
    debug!("new client connetced with session id {}", id.clone());
    Ok(Response::builder()
        .header("set-cookie", format!("session={}", id))
        .status(StatusCode::NO_CONTENT)
        .body(Body::empty()))
}

pub async fn logout(sessions: Sessions, id: String) -> Result<impl warp::Reply, Infallible> {
    debug!("client {} disconnetced", id.clone());
    // retain everything that is not id
    sessions.lock().unwrap().retain(|s| s != &id.clone());
    Ok(Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(Body::empty()))
}
