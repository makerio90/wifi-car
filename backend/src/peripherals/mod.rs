use std::{collections::HashMap, convert::Infallible, hash::Hash, sync::Arc};
mod meta;
use drivers::{
    peripheral::{self, Peripheral},
    peripherals::Peripherals,
};
use log::debug;
use std::sync::Mutex;
use warp::Filter;

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
    list(pers)
}
fn getMap(
    peripherals: PeripheralMap,
) -> impl Filter<Extract = (PeripheralMap,), Error = Infallible> + Clone {
    warp::any().map(move || peripherals.clone())
}
fn list(
    peripherals: PeripheralMap,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("peripherals" / "list")
        .and(warp::get())
        .and(getMap(peripherals))
        .and_then(meta::list)
}
