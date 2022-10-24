pub mod routes;
pub mod settings;

use clap::Parser;
use drivers::drivers::Drivers;
use log::{error, info, log_enabled, warn, Level};
use settings::Settings;
use std::env;
use std::sync::{Arc, Mutex};
use warp::Filter;
/// web interface for drivers
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// path to the config file
    #[clap(short, long, value_parser)]
    config_path: Option<String>,
}

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    info!(target: "init",
        "web based driver interface v{}",
        env!("CARGO_PKG_VERSION")
    );
    if log_enabled!(Level::Debug) {
        warn!("log::Debug Enabled. Logs may contain dangorus info. for trobleshooting use only")
    }
    let args: Args = Args::parse();
    let config_path: String = args
        .config_path
        .clone()
        .unwrap_or(format!("{}/.config/wificar.toml", env!("HOME")));
    let settings: Settings = match Settings::new(&config_path) {
        Ok(s) => s,
        Err(e) => {
            error!(target: "config","error loading config: {}", e);
            panic!()
        }
    };

    let driver = Arc::new(Mutex::new(Drivers::new(settings.driver)));

    let www = warp::fs::dir("frontend/dist/");

    let api = routes::api(driver, config_path, settings.password.get_hash().unwrap());

    warp::serve(api.or(www).with(warp::log("serv")))
        .run((settings.ip, settings.port))
        .await;
}
