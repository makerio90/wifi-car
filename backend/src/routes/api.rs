use drivers::drivers::Drivers;
use drivers::Driver;
use hyper::body::Bytes;
use log::{debug, info};
use std::convert::Infallible;
use std::fs::OpenOptions;
use std::io::Write;
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

pub async fn set_config(config_path: String, body: Bytes) -> Result<impl warp::Reply, Infallible> {
    //to_warp_error(
    OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .open(config_path)
        .unwrap()
        .write_all(&*body);
    info!("changes made to config. restart for them to take affect");
    Ok(Response::builder()
        .body("changes made. please restart the server for the changes to take affect"))
}
/*
fn to_warp_error<T>(e: Result<T, impl Serialize>) -> Result<impl warp::Reply, Infallible> {
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
