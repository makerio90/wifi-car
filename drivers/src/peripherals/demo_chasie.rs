use crate::peripheral::{
    ConfigReturn, ConfigStruct, ConfigValue, PerError, Peripheral, PeripheralError,
};
use log::info;
use serde_derive::Deserialize;
/// demo driver for testing
pub struct Demo {
    enabled: bool,
    print: String,
}
#[derive(Debug, Deserialize)]
pub struct Config {
    printme: String,
}

impl Peripheral for Demo {
    type Config<'a> = Config;
    fn init<'a>(config: Self::Config<'a>) -> Self {
        info!(target: "DemoDriver", "enabled!");
        Self {
            enabled: true,
            print: config.printme,
        }
    }
    fn config_get(&self) -> Vec<ConfigStruct> {
        vec![ConfigStruct {
            name: "print".to_string(),
            id: 0,
            value: ConfigValue::Momentary,
            discription: Some("print the value written in the config file".to_string()),
            disabled: false,
        }]
    }
    fn config_set(&mut self, id: u8, value: ConfigReturn) -> PerError<()> {
        match id {
            0 => {
                if let ConfigReturn::Momentary = value {
                    info!(target: "DemoDriver", "{}", self.print);
                    Ok(())
                } else {
                    Err(PeripheralError::WrongType)
                }
            }
            _ => Err(PeripheralError::BadId),
        }
    }

    fn rc(&self) -> Vec<String> {
        vec!["Accelerate".to_string(), "Steer".to_string()]
    }

    fn send(&mut self, values: Vec<u16>) {
        let (accelerate, steer) = if let [accelerate, steer] = values[0..3] {
            (accelerate, steer)
        } else {
            todo!()
        };

        let accelerate = accelerate as f64 * 200.0 / 65535.0 - 100.0;
        let steer = steer as f64 * 200.0 / 65535.0 - 100.0;

        let drive_speed = accelerate.abs() * 100.0;
        let steer_amount = steer.abs() * 100.0;
        let drive_dir = if accelerate.is_sign_negative() {
            "backward"
        } else {
            "forward"
        };

        let steer_dir = if steer.is_sign_negative() {
            "left"
        } else {
            "right"
        };
        info!(target: "Dummy",
            "got command to drive {} at speed {}% and steer {} {}%",
            drive_dir,
            drive_speed,
            steer_dir,
            steer_amount,
        );
    }
}
