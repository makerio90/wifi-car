mod config;
mod meta;
use drivers::{
    peripheral::{self, Peripheral},
    peripherals::Peripherals,
};
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use std::sync::Mutex;
use warp::{filters::query, Filter};

use crate::settings::Per;

type PeripheralMap = Arc<Mutex<HashMap<String, Peripherals>>>;

pub fn peripherals(
    peripherals: Vec<Per>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let mut peripheral_map: HashMap<String, Peripherals> = HashMap::new();

    for peripheral in peripherals {
        peripheral_map.insert(peripheral.name, Peripherals::init(peripheral.per));
    }

    let pers: PeripheralMap = Arc::new(Mutex::new(peripheral_map));
    list(pers.clone())
        .or(get_config(pers.clone()))
        .or(set_config(pers))
}

fn get_map(
    peripherals: PeripheralMap,
) -> impl Filter<Extract = (PeripheralMap,), Error = Infallible> + Clone {
    warp::any().map(move || peripherals.clone())
}

fn list(
    peripherals: PeripheralMap,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("peripherals" / "list")
        .and(warp::get())
        .and(get_map(peripherals))
        .and_then(meta::list)
}

fn get_config(
    peripherals: PeripheralMap,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("peripherals" / String / "config")
        .and(warp::get())
        .and(get_map(peripherals))
        .and_then(config::get)
}

fn set_config(
    peripherals: PeripheralMap,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("peripherals" / String / "config" / u8)
        // Only accept bodies smaller than 1kb...
        .and(warp::body::content_length_limit(1024))
        .and(warp::body::json())
        .and(warp::post())
        .and(get_map(peripherals))
        .and_then(config::set)
}
