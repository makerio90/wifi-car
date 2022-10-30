mod api;
mod auth;
mod websocket;

use super::Settings;
use drivers::drivers::Drivers;
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use warp::{Filter, Rejection};

type Sessions = Arc<Mutex<Vec<String>>>;

pub fn api(
    driver: Arc<Mutex<Drivers>>,
    config_path: String,
    pass: String,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let sessions: Sessions = Arc::new(Mutex::new(Vec::new()));
    info(driver.clone(), sessions.clone())
        .or(disable(driver.clone(), sessions.clone()))
        .or(enable(driver.clone(), sessions.clone()))
        .or(drive(driver.clone(), sessions.clone()))
        .or(drive_ws(driver, sessions.clone()))
        .or(login(sessions.clone(), pass))
        .or(write_config(sessions.clone(), config_path.clone()))
        .or(read_config(sessions.clone(), config_path))
}

pub fn login(
    sessions: Sessions,
    pass: String,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("auth" / "login")
        .and(warp::post())
        .and(warp::header::exact_ignore_case(
            // HACK: frontend gloo-net sets all headers to lowercase; but this should be camel case
            "authorization",
            // TODO: fix mem leak
            Box::<str>::leak(pass.into_boxed_str()),
        ))
        .and(warp::any().map(move || sessions.clone()))
        .and(warp::any().map(move || format!("{:X}", rand::random::<u32>())))
        .and_then(auth::login)
}
//TODO
/*pub fn logout(
    sessions: Sessions,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("auth" / "logout")
        .and(warp::post())
        .and(warp::any().map(move || sessions.clone()))
        .and(warp::any().map(move || -> String { warp::cookie::<String>("session").Extract }))
        .and_then(auth::logout)
}*/

pub fn read_config(
    sessions: Sessions,
    config_path: String,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "config")
        .and(warp::get())
        .and(needs_auth(sessions))
        .and(warp::fs::file(config_path))
}

pub fn write_config(
    sessions: Sessions,
    config_path: String,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "config")
        .and(warp::post())
        .and(needs_auth(sessions))
        .map(move || config_path.clone())
        .and(warp::body::content_length_limit(4096))
        .and(warp::body::bytes())
        .and_then(api::set_config)
}

pub fn enable(
    driver: Arc<Mutex<Drivers>>,
    sessions: Sessions,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "enable")
        .and(warp::post())
        .and(needs_auth(sessions))
        .and(with_driver(driver))
        .and_then(api::enable)
}

pub fn disable(
    driver: Arc<Mutex<Drivers>>,
    sessions: Sessions,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "disable")
        .and(warp::post())
        .and(needs_auth(sessions))
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
    sessions: Sessions,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "drive")
        .and(warp::query::<DriveQuery>())
        .and(warp::post())
        .and(needs_auth(sessions))
        .and(with_driver(driver))
        .and_then(api::drive)
}

pub fn drive_ws(
    driver: Arc<Mutex<Drivers>>,
    sessions: Sessions,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("ws")
        .and(warp::ws())
        .and(needs_auth(sessions))
        .and(with_driver(driver))
        .map(|ws: warp::ws::Ws, driver| {
            ws.on_upgrade(move |socket| websocket::drive(socket, driver))
        })
}

pub fn info(
    driver: Arc<Mutex<Drivers>>,
    sessions: Sessions,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "info")
        .and(warp::get())
        .and(needs_auth(sessions))
        .and(with_driver(driver))
        .and_then(api::info)
}
#[derive(Debug)]
struct Unauthorised;

impl warp::reject::Reject for Unauthorised {}

fn needs_auth(sessions: Sessions) -> impl Filter<Extract = (), Error = Rejection> + Clone {
    warp::cookie::<String>("session")
        .and(warp::any().map(move || sessions.clone()))
        .and_then(|session_id: String, sessions: Sessions| async move {
            if (*sessions.lock().unwrap()).contains(&session_id) {
                Ok(())
            } else {
                Err(warp::reject::custom(Unauthorised))
            }
        })
        .untuple_one()
}
fn with_driver(
    driver: Arc<Mutex<Drivers>>,
) -> impl Filter<Extract = (Arc<Mutex<Drivers>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || driver.clone())
}
