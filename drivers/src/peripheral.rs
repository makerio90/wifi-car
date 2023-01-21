use serde::{Deserialize, Serialize};

pub trait Peripheral {
    /// low-level config.
    /// like pin assighnments;
    type Config<'a>: Deserialize<'a>;

    ///initalises the peripheral (duh)
    fn init<'a>(config: Self::Config<'a>) -> Self;

    /// list all editable options
    fn config_get(&self) -> Vec<ConfigStruct>;

    /// runtime config
    /// things you would want to updtate at runtime.
    fn config_set(&mut self, id: u8, value: ConfigReturn) -> PerError<()>;

    /// rc values
    fn rc(&self) -> Vec<String>;

    /// send rc values
    fn send(&mut self, values: Vec<u16>);
}

#[derive(Serialize)]
pub struct ConfigStruct {
    /// name
    pub name: String,
    /// value
    pub value: ConfigValue,
    /// id
    pub id: u8,
    /// disabled
    pub disabled: bool,
    /// description
    pub discription: Option<String>,
}

/// structure for a config value
#[derive(Serialize, Deserialize)]
pub enum ConfigValue {
    /// number between min an max
    /// (min,max,step)
    Num(i32, i32, u8),
    /// string
    /// value is state
    String,
    /// true / false switch
    /// value is state
    Bool,
    /// momentary, like a reset button
    Momentary,
}

#[derive(Deserialize)]
/// config values to be returned
pub enum ConfigReturn {
    Num(i32),
    String(String),
    Bool(bool),
    Momentary,
}

pub type PerError<T> = Result<T, PeripheralError>;
pub enum PeripheralError {
    BadId,
    WrongType,
}
