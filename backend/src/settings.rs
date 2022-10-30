use config::Config;
use config::ConfigError;
use config::File;
use drivers::drivers::DriverConfig;
use log::{debug, warn};
use serde_derive::Deserialize;
use sha2::{Digest, Sha256};

#[derive(Debug, Deserialize)]
pub enum Pass {
    /// sha256 hash of password
    Hash(String),
    /// raw unhashed password. (not reccomended)
    Raw(String),
}

impl Pass {
    pub fn get_hash(&self) -> Option<String> {
        if let Pass::Hash(s) = self {
            Some(s.to_string())
        } else {
            None
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub driver: DriverConfig,
    pub ip: [u8; 4],
    pub port: u16,
    pub password: Pass,
    pub web_cam: WebCamSettings,
}

#[derive(Debug, Deserialize)]
pub struct WebCamSettings {
    #[serde(default = "default_cam")]
    pub path: String,
}
fn default_cam() -> String {
    "/dev/video0".to_string()
}
impl Settings {
    pub fn new(path: &str) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(&path))
            .build()?;
        let mut conf: Self = s.try_deserialize()?;
        if let Pass::Raw(_) = conf.password {
            warn!("avoid using raw passwords in conf file. instead use a sha256 hash")
        }
        if let Pass::Raw(raw) = conf.password {
            conf.password = Pass::Hash(format!("{:X}", Sha256::digest(raw)))
        }

        debug!("hash: {:?}", &conf.password.get_hash().unwrap());
        Ok(conf)
    }
}
