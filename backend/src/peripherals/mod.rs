use std::collections::HashMap;

use drivers::{
    peripheral::{self, Peripheral},
    peripherals::Peripherals,
};
use log::debug;

use crate::settings::Per;

pub fn peripherals(peripherals: Vec<Per>) {
    let mut peripheral_map: HashMap<String, Peripherals> = HashMap::new();

    for peripheral in peripherals {
        peripheral_map.insert(peripheral.name, Peripherals::init(peripheral.per));
    }

    for peripheral in peripheral_map {
        debug!("{}", peripheral.0)
    }
}
