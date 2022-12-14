use crate::{Driver, DriverError, Result};
use rppal::gpio::{Gpio, OutputPin};
// simple skid steer car using L298P drivers
#[derive(Debug)]
pub struct SkidSteer {
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
    gpio: Option<Gpio>,
}

impl SkidSteer {
    pub fn new(ena_pin: u8, enb_pin: u8, rva_pin: u8, rvb_pin: u8) -> Self {
        Self {
            ena_pin,
            enb_pin,
            rva_pin,
            rvb_pin,
            is_enabled: false,
            ena: None,
            enb: None,
            rva: None,
            rvb: None,
            gpio: None,
        }
    }
}
impl Driver for SkidSteer {
    fn enable(&mut self) -> Result<()> {
        self.gpio = Some(Gpio::new()?);
        self.ena = Some(
            self.gpio
                .as_mut()
                .ok_or(DriverError::ExpectedSomeFoundNone)?
                .get(self.ena_pin)?
                .into_output(),
        );
        self.enb = Some(
            self.gpio
                .as_mut()
                .ok_or(DriverError::ExpectedSomeFoundNone)?
                .get(self.enb_pin)?
                .into_output(),
        );
        self.rva = Some(
            self.gpio
                .as_mut()
                .ok_or(DriverError::ExpectedSomeFoundNone)?
                .get(self.rva_pin)?
                .into_output(),
        );
        self.rvb = Some(
            self.gpio
                .as_mut()
                .ok_or(DriverError::ExpectedSomeFoundNone)?
                .get(self.rvb_pin)?
                .into_output(),
        );
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
    fn estop(&mut self) -> Result<()> {
        if !self.is_enabled {
            return Err(DriverError::NotEnabled);
        }
        self.ena
            .as_mut()
            .ok_or(DriverError::ExpectedSomeFoundNone)?
            .set_low();
        self.enb
            .as_mut()
            .ok_or(DriverError::ExpectedSomeFoundNone)?
            .set_low();
        Ok(())
    }
    fn disable(&mut self) -> Result<()> {
        self.estop()?;
        Ok(())
    }
    fn drive(&mut self, accelerate: f64, steer: f64) -> Result<()> {
        if !(-1.0..=1.0).contains(&accelerate) || !(-1.0..=1.0).contains(&steer) {
            return Err(DriverError::OutOfRange);
        }
        if !self.is_enabled {
            return Err(DriverError::NotEnabled);
        }

        let left = (accelerate - steer).clamp(-1.0, 1.0);
        let right = (accelerate + steer).clamp(-1.0, 1.0);

        self.rva
            .as_mut()
            .ok_or(DriverError::ExpectedSomeFoundNone)?
            .write(left.is_sign_negative().into());
        self.rvb
            .as_mut()
            .ok_or(DriverError::ExpectedSomeFoundNone)?
            .write(right.is_sign_negative().into());
        self.ena
            .as_mut()
            .ok_or(DriverError::ExpectedSomeFoundNone)?
            .set_pwm_frequency(100.0, left.abs())?;
        self.enb
            .as_mut()
            .ok_or(DriverError::ExpectedSomeFoundNone)?
            .set_pwm_frequency(100.0, right.abs())?;

        Ok(())
    }
}
