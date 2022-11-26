use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub trait Peripheral {
    /// struture for data in.

    ///initalises the peripheral (duh)
    fn init<'a>() -> Self;

    /// status cmd. ~1 sentance that tell the server how its doing.
    /// think `all systems OK` and `solar cell at 15.7V` kind of think. in an error
    /// it would say something like `!system oflline! short detected`
    /// NOTE: do not just pipe stderror out, this is a status line not a log.
    /// NOTE: this will be called very rapidly, make it quick.
    fn status(&self) -> Result<String, String>;

    /// current values being used. make it quick!
    fn values<'a>(&self) -> HashMap<String, Value>;

    /// data in
    /// bout what you'd expect.
    /// ran in a seprate thread, take as mutch time as you need.
    /// TODO: beter errors
    fn update<'a>(&mut self, in_data: HashMap<String, Value>) -> HashMap<String, Value>;
}

pub struct Example {
    b: i32,
    a: i32,
}

impl Peripheral for Example {
    fn init() -> Self {
        Self { a: 0, b: 0 }
    }

    fn values<'a>(&self) -> HashMap<String, Value> {
        Vec<("num a", Value::Num { value: self.a, min: i32::MIN, max: i32::MAX}),
        ("num b", Value::Num { value: self.b, min: i32::MIN, max: i32::MAX})>.into()
    }

    fn status(&self) -> Result<String, String> {
        Ok("lookin Good!".to_string())
    }

    fn update<'a>(&mut self, in_data: HashMap<String, Value>) -> HashMap<String, Value>{
        *self = Example { a:
                      in_data.get("num a").unwrap().value, b:
                      in_data.get("num b").unwrap().value};
        vec![].into()
    }
}

enum Value {
    Num { value: i32, min: u16, max: u16 },
}
