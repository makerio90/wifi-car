use config::Config;
use config::ConfigError;
use config::File;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub driver: String,
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
