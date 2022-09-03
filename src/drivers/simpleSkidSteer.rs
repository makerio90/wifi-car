use crate::drivers::{Driver, DriverError};
use rppal::gpio::{Gpio, OutputPin};
// simple skid steer car using L298P drivers
struct SkidSteer {
    /// motor driver a enable pin
    /// pwm support is not yet supported, softpwm is used.
    /// TODO: allow for pwm enable pins
    pub ena_pin: u8,
    /// motor driver b enable pin
    /// hardware pwm is not yet supported, softpwm is used.
    /// TODO: allow for pwm enable pins
    pub enb_pin: u8,
    /// motor driver a reverse pin
    pub rva_pin: u8,
    /// motor driver b reverse pin
    pub rvb_pin: u8,
    /// is the driver enabled
    is_enabled: bool,
    /// output pin object for driver a enable pin
    ena: Option<OutputPin>,
    /// output pin object for driver b enable pin
    enb: Option<OutputPin>,
    /// output pin object for driver a reverse pin
    rva: Option<OutputPin>,
    /// output pin object for driver b reverse pin
    rvb: Option<OutputPin>,
    /// gpio object
    gpio: Gpio,
}

impl Driver for SkidSteer {
    fn enable(&mut self) -> Result<(), DriverError> {
        self.gpio = Gpio::new()?;
        self.ena = Some(self.gpio.get(self.ena_pin)?.into_output());
        self.enb = Some(self.gpio.get(self.enb_pin)?.into_output());
        self.rva = Some(self.gpio.get(self.rva_pin)?.into_output());
        self.rva = Some(self.gpio.get(self.rva_pin)?.into_output());
        self.is_enabled = true;
        Ok(())
    }
    fn is_ready(&self) -> bool {
        self.is_enabled
    }
    fn has_break(&self) -> bool {
        false
    }
    fn is_proportional(&self) -> (bool, bool) {
        (true, true)
    }
    fn estop(&mut self) -> Result<(), DriverError> {
        if self.is_enabled {
            return Err(DriverError::NotEnabled);
        }
        // FIXME
        self.ena
            .expect("is_enabled was true but no enabale struct found")
            .set_low();
        self.enb
            .expect("is_enabled was true but no emable struct found")
            .set_low();
        Ok(())
    }
    fn disable(mut self) -> Result<(), DriverError> {
        self.estop()?;
        Ok(())
    }
    fn drive(&mut self, accelerate: f64, steer: f64) -> Result<(), DriverError> {
        if !(0.0..1.0).contains(&accelerate) || !(-1.0..1.0).contains(&steer) {
            return Err(DriverError::OutOfRange);
        }
        Ok(())
    }
}
