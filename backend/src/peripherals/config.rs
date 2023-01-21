use std::convert::Infallible;

use drivers::peripheral::{ConfigStruct, Peripheral};

use super::PeripheralMap;

pub async fn get(
    peripheral: String,
    peripherals: PeripheralMap,
) -> Result<impl warp::Reply, Infallible> {
    let peripherals = peripherals.lock().unwrap();
    let config = peripherals[&peripheral].config_get();
    Ok(warp::reply::json(&config))
}

pub async fn set(
    peripheral: String,
    id: u8,
    config: ConfigStruct,
    peripherals: PeripheralMap,
) -> Result<impl warp::Reply, Infallible> {
    let mut peripherals = peripherals.lock().unwrap();
    let config = peripherals
        .get_mut(&peripheral)
        .unwrap()
        .config_set(id, config);
    Ok(warp::reply())
}
