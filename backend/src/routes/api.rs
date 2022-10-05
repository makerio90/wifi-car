use drivers::drivers::Drivers;
use drivers::Driver;
use log::debug;
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use warp::http::StatusCode;

pub async fn enable(driver: Arc<Mutex<Drivers>>) -> Result<impl warp::Reply, Infallible> {
    debug!(target: "api", "waiting for driver lock");
    let mut driver = driver.lock().unwrap();
    debug!(target: "api", "got driver lock");
    match (*driver).enable() {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn disable(driver: Arc<Mutex<Drivers>>) -> Result<impl warp::Reply, Infallible> {
    debug!(target: "api", "waiting for driver lock");
    let mut driver = driver.lock().unwrap();
    debug!(target: "api", "got driver lock");
    match (*driver).disable() {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn drive(
    params: super::DriveQuery,
    driver: Arc<Mutex<Drivers>>,
) -> Result<impl warp::Reply, Infallible> {
    debug!(target: "api", "waiting for driver lock");
    let mut driver = driver.lock().unwrap();
    debug!(target: "api", "got driver lock");
    match (*driver).drive(params.accelerate, params.steer) {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn info(driver: Arc<Mutex<Drivers>>) -> Result<impl warp::Reply, Infallible> {
    debug!(target: "api", "waiting for driver lock");
    let driver = driver.lock().unwrap();
    debug!(target: "api", "got driver lock");
    Ok(warp::reply::json(&(*driver).is_ready()))
}
