use crate::peripheral::{ConfigReturn, ConfigStruct, PerError, Peripheral, PeripheralError};
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
	ena: OutputPin,
	/// output pin object for driver b enable pin
	enb: OutputPin,
	/// output pin object for driver a reverse pin
	rva: OutputPin,
	/// output pin object for driver b reverse pin
	rvb: OutputPin,
	/// gpio object
	gpio: Gpio,
}
#[derive(Debug, Deserialize)]
pub struct Config {
	ena_pin: u8,
	enb_pin: u8,
	rva_pin: u8,
	rvb_pin: u8,
}

impl Peripheral for SkidSteer {
	type Config<'a> = Config;
	fn init<'a>(config: Self::Config<'a>) -> PerError<Self> {
		let gpio = Gpio::new()?;
		let ena = gpio.get(config.ena_pin)?.into_output();
		let enb = gpio.get(config.enb_pin)?.into_output();
		let rva = gpio.get(config.rva_pin)?.into_output();
		let rvb = gpio.get(config.rvb_pin)?.into_output();
		Ok(Self {
			ena_pin: config.ena_pin,
			enb_pin: config.enb_pin,
			rva_pin: config.rva_pin,
			rvb_pin: config.rvb_pin,
			ena,
			enb,
			rva,
			rvb,
			gpio,
		})
	}
	fn config_get(&self) -> Vec<ConfigStruct> {
		vec![]
	}
	fn config_set(&mut self, id: u8, value: ConfigReturn) -> PerError<()> {
		Err(PeripheralError::BadId)
	}

	fn rc(&self) -> Vec<String> {
		vec!["accelerate".to_string(), "steer".to_string()]
	}

	fn send(&mut self, values: Vec<u16>) -> PerError<()> {
		let (accelerate, steer) = if let [accelerate, steer] = values[0..3] {
			(accelerate, steer)
		} else {
			todo!()
		};

		let accelerate = accelerate as f64 * 200.0 / 65535.0 - 100.0;
		let steer = steer as f64 * 200.0 / 65535.0 - 100.0;

		let left = (accelerate - steer).clamp(-1.0, 1.0);
		let right = (accelerate + steer).clamp(-1.0, 1.0);

		self.rva.write(left.is_sign_negative().into());
		self.rvb.write(right.is_sign_negative().into());
		self.ena.set_pwm_frequency(100.0, left.abs())?;
		self.enb.set_pwm_frequency(100.0, right.abs())?;

		Ok(())
	}
}
