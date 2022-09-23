use config::Config;
use config::ConfigError;
use config::File;
use log::info;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub driver: String,
}

impl Settings {
    pub fn new(path: String) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(&path))
            .build()?;
        info!(
            target : "config",
            "using driver: {:?}",
            s.get::<String>("driver").unwrap_or("None".to_string())
        );
        s.try_deserialize()
    }
}
