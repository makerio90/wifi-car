use config::Config;
use config::ConfigError;
use config::File;
use drivers::drivers::DriverConfig;
use serde_derive::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub driver: DriverConfig,
    pub ip: String,
    pub port: u16,
}

impl Settings {
    pub fn new(path: String) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(&path))
            .build()?;
        s.try_deserialize()
    }
}
