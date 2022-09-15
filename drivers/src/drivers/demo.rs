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
        if !(0.0..1.1).contains(&accelerate) || !(-1.0..1.1).contains(&steer) {
            return Err(DriverError::OutOfRange);
        }
        if self.enabled {
            return Err(DriverError::NotEnabled);
        }
        let speed = accelerate * 100.0;
        let dir = if steer.is_sign_negative() {
            "left"
        } else {
            "right"
        };
        let amount = steer.abs() * 100.0;
        println!(
            "got command to drive at speed {}% and steer {} {}",
            speed, dir, amount
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
        println!("disabled");
        Ok(())
    }
}
