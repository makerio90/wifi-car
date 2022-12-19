use serde::{Deserialize, Serialize};

pub trait Peripheral {
    /// low-level config.
    /// like pin assighnments;
    type Config<'a>: Deserialize<'a>;

    ///initalises the peripheral (duh)
    fn init<'a>(config: Self::Config<'a>) -> Self;

    /// list all editable options
    fn config_get(&self) -> Vec<Value<ConfigValue>>;

    /// runtime config
    /// things you would want to updtate at runtime.
    fn config_set(&mut self, id: u8, value: ConfigValue) -> PerError<()>;

    /// rc values
    fn rc(&self) -> Vec<Value<RcValue>>;

    /// send rc values
    /// it is only possible to update them all at once
    /// they will be sent in `id` order
    fn send(&mut self, values: Vec<RcValue>);
}
#[derive(Serialize)]
pub enum RcValue {
    /// analog value between 0 - 1
    Analog(f64),
    /// momentary value
    Momentary,
    /// toggle switch
    Continus(bool),
}

#[derive(Serialize)]
pub struct Value<T> {
    /// name
    pub name: String,
    /// value
    pub value: T,
    /// id
    pub id: u8,
}

#[derive(Serialize, Deserialize)]
pub enum ConfigValue {
    /// number between min an max
    Num { value: i32, min: u16, max: u16 },
    /// string
    /// value is default
    String(String),
    /// true / false switch
    /// value is default
    Bool(bool),
    /// momentary, like a reset button
    Momentary,
}
pub type PerError<T> = Result<T, PeripheralError>;
pub enum PeripheralError {
    BadId,
    WrongType,
}
