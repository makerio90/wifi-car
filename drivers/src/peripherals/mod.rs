pub mod demo;

use serde::Deserialize;

use crate::peripheral::{ConfigValue, Peripheral, RcValue, Value};

pub enum Peripherals {
    Demo(demo::Demo),
}
#[derive(Debug, Deserialize)]
pub enum PeripheralConfig {
    Demo(demo::DemoConfig),
}

impl Peripheral for Peripherals {
    type Config<'a> = PeripheralConfig;
    fn init<'a>(config: Self::Config<'a>) -> Self {
        match config {
            PeripheralConfig::Demo(cfg) => Peripherals::Demo(demo::Demo::init(cfg)),
        }
    }
    fn config_set(
        &mut self,
        id: u8,
        value: crate::peripheral::ConfigValue,
    ) -> crate::peripheral::PerError<()> {
        match self {
            Peripherals::Demo(per) => per.config_set(id, value),
        }
    }
    fn config_get(&self) -> Vec<Value<ConfigValue>> {
        match self {
            Peripherals::Demo(per) => per.config_get(),
        }
    }
    fn rc(&self) -> Vec<Value<RcValue>> {
        match self {
            Peripherals::Demo(per) => per.rc(),
        }
    }
    fn send(&mut self, values: Vec<RcValue>) {
        match self {
            Peripherals::Demo(per) => per.send(values),
        }
    }
}
