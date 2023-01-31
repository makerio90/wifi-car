use hyper::body::Bytes;
use log::{debug, info};
use std::convert::Infallible;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};
use warp::http::{Response, StatusCode};

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
