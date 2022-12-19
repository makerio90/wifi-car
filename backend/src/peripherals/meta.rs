use super::PeripheralMap;
use drivers::peripheral::Peripheral;
use std::convert::Infallible;

pub async fn list(pers: PeripheralMap) -> Result<impl warp::Reply, Infallible> {
    let pers = pers.lock().unwrap();
    let names: Vec<&String> = (*pers).iter().map(|per| per.0).collect();
    Ok(warp::reply::json(&names))
}

pub async fn get_map(per: String, pers: PeripheralMap) -> Result<impl warp::Reply, Infallible> {
    let pers = pers.lock().unwrap();
    Ok(warp::reply::json(&pers[&per].rc()))
}
