use config::Config;
use config::ConfigError;
use config::File;
use drivers::drivers::DriverConfig;
use log::{log_enabled, warn, Level};
use serde_derive::Deserialize;
use sha2::{Digest, Sha256};

#[derive(Debug, Deserialize)]
pub enum Pass {
    /// sha256 hash of password
    Hash(String),
    /// raw unhashed password. (not reccomended)
    Raw(String),
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub driver: DriverConfig,
    pub ip: [u8; 4],
    pub port: u16,
    pub password: Pass,
}

impl Settings {
    pub fn new(path: String) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(&path))
            .build()?;
        let mut conf: Self = s.try_deserialize()?;
        if let Pass::Raw(_) = conf.password {
            warn!("avoid using raw passwords in conf file. instead use a sha256 hash")
        }
        if let Pass::Raw(mut raw) = conf.password {
            conf.password = Pass::Hash(format!("{:X}", Sha256::digest(raw)))
        }
        Ok(conf)
    }
}
