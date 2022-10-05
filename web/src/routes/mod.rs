mod api;

use drivers::drivers::Drivers;
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use warp::Filter;

pub fn api(
    driver: Arc<Mutex<Drivers>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    info(driver.clone())
        .or(disable(driver.clone()))
        .or(enable(driver.clone()))
        .or(drive(driver.clone()))
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

#[derive(Deserialize)]
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

pub fn info(
    driver: Arc<Mutex<Drivers>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "info")
        .and(warp::get())
        .and(with_driver(driver))
        .and_then(api::info)
}

fn with_driver(
    driver: Arc<Mutex<Drivers>>,
) -> impl Filter<Extract = (Arc<Mutex<Drivers>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || driver.clone())
}
