pub mod demo_chasie;
pub mod demo_peripheral;
pub mod simple_skid_steer;
use crate::peripheral::{ConfigStruct, PerError, Peripheral};
use serde::Deserialize;

pub enum Peripherals {
    Demo(demo_peripheral::Demo),
}

#[derive(Debug, Deserialize)]
pub enum PeripheralConfig {
    Demo(demo_peripheral::DemoConfig),
}

impl Peripheral for Peripherals {
    type Config<'a> = PeripheralConfig;
    fn init<'a>(config: Self::Config<'a>) -> PerError<Self> {
        match config {
            PeripheralConfig::Demo(cfg) => Peripherals::Demo(demo_peripheral::Demo::init(cfg)?),
        }
    }
    fn config_set(
        &mut self,
        id: u8,
        value: crate::peripheral::ConfigReturn,
    ) -> crate::peripheral::PerError<()> {
        match self {
            Peripherals::Demo(per) => per.config_set(id, value),
        }
    }
    fn config_get(&self) -> Vec<ConfigStruct> {
        match self {
            Peripherals::Demo(per) => per.config_get(),
        }
    }
    fn rc(&self) -> Vec<String> {
        match self {
            Peripherals::Demo(per) => per.rc(),
        }
    }
    fn send(&mut self, values: Vec<u16>) -> PerError<()> {
        match self {
            Peripherals::Demo(per) => per.send(values),
        }
    }
}
