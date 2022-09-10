use std::result;
mod demo;
mod simple_skid_steer;

pub enum Drivers {
    SimpleSkidSteer(simple_skid_steer::SkidSteer),
}

trait Driver {
    /// enable the car. do whatever neccicary to get the driver ready to drive
    /// run this before trying to run any other functions.
    fn enable(&mut self) -> Result<()>;
    /// is the driver enabled and ready for communication?
    fn is_ready(&self) -> bool;
    /// drive funtion.
    /// vales of avcellerate and brake are capped at +-1,
    /// any values grater than this will be capped at 1,
    /// so a value of 2 would have the same effect as a value of 1.
    /// 1 for full speed ahead, 0 for no change, -1 is reverse/brake.
    /// -1 for full steer left, 1 for full speed right.
    fn drive(&mut self, accelerate: f64, steer: f64) -> Result<()>;
    /// stop the car, no mater what.
    /// this is sort of like an e-stop.
    /// this should get pulled as a fialsafe.
    fn estop(&mut self) -> Result<()>;
    /// returns true if the vichle has a brake.
    fn has_break(&self) -> bool;
    /// returns a tuple for if the veicle has proportional controls
    /// (has_proportional_steering,has_proportional_drive)
    fn is_proportional(&self) -> (bool, bool);
    /// soft stop.
    /// this is for a controlled stutdown, and is as calm as possible
    /// this is for when you hit the 'end' button
    fn disable(&mut self) -> Result<()>;
}
/// any error that can return of attempting to use the above funtions
enum DriverError {
    /// you tried to run a function but the driver was not enabled.
    /// dont `panic!` just enable the driver.
    NotEnabled,
    /// you tried to pass a function a number that was out of its range.
    /// it will usualy still work, just not how you may expect it to.
    OutOfRange,
    /// internal error, expected a value of Some<T> found None.
    /// best to panic!
    ExpectedSomeFoundNone,
    /// gpio error
    Gpio(rppal::gpio::Error),
}
/// TODO: better way to impl this?
impl From<rppal::gpio::Error> for DriverError {
    fn from(e: rppal::gpio::Error) -> DriverError {
        DriverError::Gpio(e)
    }
}
type Result<T> = result::Result<T, DriverError>;
