use crate::peripheral::{self, ConfigValue, PerError, Peripheral, RcValue, Value};
use log::info;
use serde::Deserialize;

pub struct Demo {
    a: u32,
    b: u32,
    string: String,
}
#[derive(Debug, Deserialize)]
pub struct DemoConfig {
    /// text to print at startup
    infocmd: String,
}
impl Peripheral for Demo {
    type Config<'a> = DemoConfig;
    fn init<'a>(config: Self::Config<'a>) -> Self {
        info!(target: "DemoPeripheral", "starting up! {}", config.infocmd);
        Self {
            a: 0,
            b: 0,
            string: String::new(),
        }
    }
    fn config_get(&self) -> Vec<Value<ConfigValue>> {
        vec![
            Value {
                name: "a value".to_string(),
                id: 0,
                value: ConfigValue::Num {
                    value: self.a as i32,
                    min: 0,
                    max: 100,
                },
            },
            Value {
                name: "what to print".to_string(),
                id: 1,
                value: ConfigValue::String(self.string.clone()),
            },
            Value {
                name: "print".to_string(),
                id: 2,
                value: ConfigValue::Momentary,
            },
        ]
    }

    fn config_set(&mut self, id: u8, value: ConfigValue) -> PerError<()> {
        match id {
            0 => {
                if let ConfigValue::Num {
                    value,
                    min: _,
                    max: _,
                } = value
                {
                    self.a = value as u32;
                    Ok(())
                } else {
                    Err(peripheral::PeripheralError::WrongType)
                }
            }
            1 => {
                if let ConfigValue::String(string) = value {
                    self.string = string;
                    info!(target: "DemoPeripheral", "changed `string` to {}", self.string);
                    Ok(())
                } else {
                    Err(peripheral::PeripheralError::WrongType)
                }
            }
            _ => Err(peripheral::PeripheralError::BadId),
        }
    }

    fn rc(&self) -> Vec<Value<crate::peripheral::RcValue>> {
        vec![
            Value {
                name: String::from("b value"),
                id: 0,
                value: peripheral::RcValue::Analog(self.b as f64),
            },
            Value {
                name: String::from("print"),
                id: 1,
                value: peripheral::RcValue::Momentary,
            },
        ]
    }

    fn send(&mut self, values: Vec<crate::peripheral::RcValue>) {
        if let RcValue::Analog(value) = values[0] {
            self.b = value as u32
        }
        if let RcValue::Momentary = values[1] {
            info!(target: "DemoPeripheral", "got cmd to print {}", self.string)
        }
    }
}
