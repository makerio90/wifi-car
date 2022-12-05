use std::convert::Infallible;

use super::PeripheralMap;

pub async fn list(pers: PeripheralMap) -> Result<impl warp::Reply, Infallible> {
    let pers = pers.lock().unwrap();
    let names: Vec<&String> = (*pers).iter().map(|per| per.0).collect();
    Ok(warp::reply::json(&names))
}
