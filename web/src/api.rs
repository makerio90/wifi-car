use drivers::drivers::Drivers;
use drivers::Driver;
use log::debug;
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use warp::Filter;
pub fn api(
    driver: Arc<Mutex<Drivers>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    info(driver.clone())
}
pub fn info(
    driver: Arc<Mutex<Drivers>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    //TODO: proper routing
    warp::path!("api")
        .and(warp::get())
        .and(with_db(driver))
        .and_then(get_info)
}
async fn get_info(driver: Arc<Mutex<Drivers>>) -> Result<impl warp::Reply, Infallible> {
    debug!(target: "api", "waiting for driver lock");
    let driver = driver.lock().unwrap();
    debug!(target: "api", "got driver lock");
    //TODO: display more info
    Ok(warp::reply::json(&(*driver).is_ready()))
}
fn with_db(
    driver: Arc<Mutex<Drivers>>,
) -> impl Filter<Extract = (Arc<Mutex<Drivers>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || driver.clone())
}
