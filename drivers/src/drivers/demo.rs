use crate::{Driver, DriverError, Result};

/// demo driver for testing
pub struct Demo {
    enabled: bool,
}
impl Demo {
    pub fn new() -> Self {
        Demo { enabled: false }
    }
}
impl Default for Demo {
    fn default() -> Self {
        Self::new()
    }
}
impl Driver for Demo {
    fn enable(&mut self) -> Result<()> {
        self.enabled = true;
        println!("dummy driver enabled!");
        Ok(())
    }
    fn is_ready(&self) -> bool {
        self.enabled
    }
    fn drive(&mut self, accelerate: f64, steer: f64) -> Result<()> {
        if !(-1.0..1.1).contains(&accelerate) || !(-1.0..1.1).contains(&steer) {
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
        println!(
            "got command to drive {} at speed {}% and steer {} {}",
            drive_dir, drive_speed, steer_dir, steer_amount
        );
        Ok(())
    }
    fn estop(&mut self) -> Result<()> {
        println!("estop pulled");
        Ok(())
    }
    fn has_break(&self) -> bool {
        true
    }
    fn is_proportional(&self) -> (bool, bool) {
        (true, true)
    }
    fn disable(&mut self) -> Result<()> {
        self.enabled = false;
        println!("disabled");
        Ok(())
    }
}
