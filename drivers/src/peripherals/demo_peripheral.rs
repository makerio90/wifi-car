use crate::peripheral::{self, ConfigReturn, ConfigStruct, ConfigValue, PerError, Peripheral};
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
    fn init<'a>(config: Self::Config<'a>) -> PerError<Self> {
        info!(target: "DemoPeripheral", "starting up! {}", config.infocmd);
        Ok(Self {
            a: 0,
            b: 0,
            string: String::new(),
        })
    }
    fn config_get(&self) -> Vec<ConfigStruct> {
        vec![
            ConfigStruct {
                name: "a value".to_string(),
                id: 0,
                value: ConfigValue::Num(0, 100, 1),
                discription: Some("anywhere from 0 to 100".to_string()),
                disabled: false,
            },
            ConfigStruct {
                name: "what to print".to_string(),
                id: 1,
                value: ConfigValue::String,
                discription: None,
                disabled: false,
            },
            ConfigStruct {
                name: "print".to_string(),
                id: 2,
                value: ConfigValue::Momentary,
                discription: None,
                disabled: self.a > 50,
            },
        ]
    }

    fn config_set(&mut self, id: u8, value: ConfigReturn) -> PerError<()> {
        match id {
            0 => {
                if let ConfigReturn::Num(x) = value {
                    self.a = x as u32;
                    Ok(())
                } else {
                    Err(peripheral::PeripheralError::WrongType)
                }
            }
            1 => {
                if let ConfigReturn::String(string) = value {
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

    fn rc(&self) -> Vec<String> {
        vec!["b value".to_string(), "Print".to_string()]
    }

    fn send(&mut self, values: Vec<u16>) -> PerError<()> {
        self.b = values[0] as u32;
        if values[1] > 2000 {
            info!(target: "DemoPeripheral", "got cmd to print {}", self.string)
        }
        Ok(())
    }
}
