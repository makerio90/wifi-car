use serde::{Deserialize, Serialize};

pub trait Peripheral {
    /// struture for data in.
    type InData<'a>: Deserialize<'a>;
    /// structure for output data.
    type OutData: Serialize;

    ///initalises the peripheral (duh)
    fn init() -> Self;

    /// status cmd. ~1 sentance that tell the server how its doing.
    /// think `all systems OK` and `solar cell at 15.7V` kind of think. in an error
    /// it would say something like `!system oflline! short detected`
    /// NOTE: do not just pipe stderror out, this is a status line not a log.
    /// NOTE: this will be called very rapidly, make it quick.
    fn status(self) -> Result<String, String>;

    /// data in
    /// bout what you'd expect.
    /// ran in a seprate thread, take as mutch time as you need.
    fn update<'a>(self, in_data: Self::InData<'a>) -> Self::OutData;
}

pub struct Example {
    b: i32,
    a: i32,
}

impl Peripheral for Example {
    type InData<'a> = (i32, i32);
    type OutData = i32;

    fn init() -> Self {
        Self { a: 0, b: 0 }
    }

    fn status(self) -> Result<String, String> {
        Ok("lookin Good!".to_string())
    }

    fn update<'a>(self, in_data: Self::InData<'a>) -> Self::OutData {
        in_data.0 + in_data.1
    }
}
