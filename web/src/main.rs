pub mod routes;
pub mod settings;

use clap::Parser;
use drivers::drivers::Drivers;
use log::{error, info, LevelFilter};
use settings::Settings;
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

#[tokio::main] // or #[tokio::main]
async fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    info!(target: "init",
        "web based driver interface v{}",
        env!("CARGO_PKG_VERSION")
    );
    let args: Args = Args::parse();
    let config_path: String = args
        .config_path
        .clone()
        .unwrap_or(format!("{}/.config/wificar.toml", env!("HOME")));
    let settings: settings::Settings = match Settings::new(config_path) {
        Ok(s) => s,
        Err(e) => {
            error!(target: "config","error loading config: {}", e);
            panic!()
        }
    };

    let driver = Arc::new(Mutex::new(Drivers::new(settings.driver)));

    let api = routes::api(driver).with(warp::log("api"));
    warp::serve(api).run((settings.ip, settings.port)).await;
}
