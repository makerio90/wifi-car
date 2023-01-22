use crate::driver::{Driver, DriverError, Result};
use rppal::gpio::{Gpio, OutputPin};
use serde_derive::Deserialize;
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
#[derive(Debug, Deserialize)]
pub struct Config {
    ena_pin: u8,
    enb_pin: u8,
    rva_pin: u8,
    rvb_pin: u8,
}

impl Peripheral for SkidSteer {
    type Config<'a> = DemoConfig;
    fn init<'a>(config: Self::Config<'a>) -> Self {
        let gpio = Gpio::new()?;
        let ena = Some(
            gpio.as_mut()
                .ok_or(DriverError::ExpectedSomeFoundNone)?
                .get(self.ena_pin)?
                .into_output(),
        );
        let enb = Some(
            gpio.as_mut()
                .ok_or(DriverError::ExpectedSomeFoundNone)?
                .get(self.enb_pin)?
                .into_output(),
        );
        let rva = Some(
            gpio.as_mut()
                .ok_or(DriverError::ExpectedSomeFoundNone)?
                .get(self.rva_pin)?
                .into_output(),
        );
        let rvb = Some(
            gpio.as_mut()
                .ok_or(DriverError::ExpectedSomeFoundNone)?
                .get(self.rvb_pin)?
                .into_output(),
        );
        Self {
            ena_pin: config.ena_pin,
            enb_pin: config.enb_pin,
            rva_pin: config.rva_pin,
            rvb_pin: config.rvb_pin,
            ena: None,
            enb: None,
            rva: None,
            rvb: None,
            gpio: None,
        }
    }
    fn is_ready(&self) -> bool {
        self.is_enabled
    }
    fn disable(&mut self) -> Result<()> {
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
        self.is_enabled = false;
        self.gpio = None;
        self.ena = None;
        self.enb = None;
        self.rva = None;
        self.rvb = None;
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
