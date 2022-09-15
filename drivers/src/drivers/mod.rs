use crate::{Driver, Result};
pub mod demo;
pub mod simple_skid_steer;
pub enum Drivers {
    SimpleSkidSteer(simple_skid_steer::SkidSteer),
    Demo(demo::Demo),
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
    fn estop(&mut self) -> Result<()> {
        match self {
            Drivers::SimpleSkidSteer(s) => s.estop(),
            Drivers::Demo(s) => s.estop(),
        }
    }
    fn has_break(&self) -> bool {
        match self {
            Drivers::SimpleSkidSteer(s) => s.has_break(),
            Drivers::Demo(s) => s.has_break(),
        }
    }
    fn is_proportional(&self) -> (bool, bool) {
        match self {
            Drivers::SimpleSkidSteer(s) => s.is_proportional(),
            Drivers::Demo(s) => s.is_proportional(),
        }
    }
    fn disable(&mut self) -> Result<()> {
        match self {
            Drivers::SimpleSkidSteer(s) => s.disable(),
            Drivers::Demo(s) => s.disable(),
        }
    }
}
