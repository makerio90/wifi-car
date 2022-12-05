use crate::driver::{Driver, Result};
pub mod demo;
pub mod simple_skid_steer;
use serde_derive::Deserialize;

pub enum Drivers {
    SimpleSkidSteer(simple_skid_steer::SkidSteer),
    Demo(demo::Demo),
}
#[derive(Debug, Deserialize)]
pub enum DriverConfig {
    SimpleSkidSteer(simple_skid_steer::Config),
    Demo(demo::Config),
}
impl Drivers {
    pub fn new(c: DriverConfig) -> Self {
        match c {
            DriverConfig::SimpleSkidSteer(c) => {
                Drivers::SimpleSkidSteer(simple_skid_steer::SkidSteer::new(c))
            }
            DriverConfig::Demo(c) => Drivers::Demo(demo::Demo::new(c)),
        }
    }
}
impl Driver for Drivers {
    fn enable(&mut self) -> Result<()> {
        match self {
            Drivers::SimpleSkidSteer(s) => s.enable(),
            Drivers::Demo(s) => s.enable(),
        }
    }
    fn is_ready(&self) -> bool {
        match self {
            Drivers::SimpleSkidSteer(s) => s.is_ready(),
            Drivers::Demo(s) => s.is_ready(),
        }
    }
    fn drive(&mut self, accelerate: f64, steer: f64) -> Result<()> {
        match self {
            Drivers::SimpleSkidSteer(s) => s.drive(accelerate, steer),
            Drivers::Demo(s) => s.drive(accelerate, steer),
        }
    }
    fn disable(&mut self) -> Result<()> {
        match self {
            Drivers::SimpleSkidSteer(s) => s.disable(),
            Drivers::Demo(s) => s.disable(),
        }
    }
}
