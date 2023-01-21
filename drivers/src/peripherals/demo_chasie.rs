use crate::driver::{Driver, DriverError, Result};
use log::info;
use serde_derive::Deserialize;
/// demo driver for testing
pub struct Demo {
    enabled: bool,
}
#[derive(Debug, Deserialize)]
pub struct Config {
    printme: String,
}

impl Demo {
    pub fn new(config: Config) -> Self {
        info!(target: "DemoDriver", "{}", config.printme);
        Demo { enabled: false }
    }
}

impl Default for Demo {
    fn default() -> Self {
        Self::new(Config {
            printme: "new from default trait".to_string(),
        })
    }
}
impl Peripheral for Demo {
    fn enable(&mut self) -> Result<()> {
        self.enabled = true;
        info!(target: "DemoDriver", "enabled!");
        Ok(())
    }
    fn is_ready(&self) -> bool {
        self.enabled
    }
    fn drive(&mut self, accelerate: f64, steer: f64) -> Result<()> {
        if !(-1.0..=1.0).contains(&accelerate) || !(-1.0..=1.0).contains(&steer) {
            return Err(DriverError::OutOfRange);
        }
        if !self.enabled {
            return Err(DriverError::NotEnabled);
        }
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
        Ok(())
    }
    fn disable(&mut self) -> Result<()> {
        self.enabled = false;
        info!(target: "Dummy", "disabled;");
        Ok(())
    }
}
