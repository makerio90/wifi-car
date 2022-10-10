mod api;
mod websocket;

use drivers::drivers::Drivers;
use log::info;
use rand::distributions::{Alphanumeric, DistString};
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use warp::{Filter, Rejection};
type Sessions = Arc<Mutex<Vec<String>>>;
pub fn api(
    driver: Arc<Mutex<Drivers>>,
    //apiKey: &str,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    info(driver.clone())
        .or(disable(driver.clone()))
        .or(enable(driver.clone()))
        .or(drive(driver.clone()))
        .or(drive_ws(driver))
}

pub fn enable(
    driver: Arc<Mutex<Drivers>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "enable")
        .and(warp::post())
        .and(with_driver(driver))
        .and_then(api::enable)
}

pub fn disable(
    driver: Arc<Mutex<Drivers>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "disable")
        .and(warp::post())
        .and(with_driver(driver))
        .and_then(api::disable)
}

#[derive(Debug, Deserialize)]
pub struct DriveQuery {
    accelerate: f64,
    steer: f64,
}

pub fn drive(
    driver: Arc<Mutex<Drivers>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "drive")
        .and(warp::query::<DriveQuery>())
        .and(warp::post())
        .and(with_driver(driver))
        .and_then(api::drive)
}

pub fn drive_ws(
    driver: Arc<Mutex<Drivers>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("ws")
        .and(warp::ws())
        .and(with_driver(driver))
        .map(|ws: warp::ws::Ws, driver| {
            ws.on_upgrade(move |socket| websocket::drive(socket, driver))
        })
}

pub fn info(
    driver: Arc<Mutex<Drivers>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "info")
        .and(warp::get())
        .and(with_driver(driver))
        .and_then(api::info)
}
#[derive(Debug)]
struct Unauthorised;

impl warp::reject::Reject for Unauthorised {}

fn needs_auth(sessions: Sessions) -> impl Filter<Extract = ((),), Error = Rejection> + Clone {
    warp::cookie("session")
        .and(warp::any().map(move || sessions.clone()))
        .and_then(|session_id: String, sessions: Sessions| async move {
            if (*sessions.lock().unwrap()).contains(&session_id) {
                Ok(())
            } else {
                Err(warp::reject::custom(Unauthorised))
            }
        })
}
fn with_driver(
    driver: Arc<Mutex<Drivers>>,
) -> impl Filter<Extract = (Arc<Mutex<Drivers>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || driver.clone())
}
