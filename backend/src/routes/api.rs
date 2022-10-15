use drivers::drivers::Drivers;
use drivers::{Driver, DriverError};
use hyper::Body;
use log::{debug, error};
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use warp::http::{Response, StatusCode};

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
/*
 * FIXME
fn to_warp_error(e: Result<T, DriverError>) -> Result<impl warp::Reply, Infallible> {
    match e {
        Ok(_) => Ok(Response::builder().body(Body::empty())),
        Err(e) => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(
                serde_json::to_string(&e)
                    .expect("unexpected error with serde_json")
                    .into(),
            )),
    }
}
*/
