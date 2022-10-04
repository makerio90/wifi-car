pub mod api;
pub mod settings;

use clap::Parser;
use drivers::drivers::{demo::Demo, simple_skid_steer::SkidSteer, Drivers};
use lazy_static::lazy_static;
use log::{error, info, trace};
use settings::Settings;
use std::sync::{Arc, Mutex};
use warp::Filter;
/// web interface for drivers
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// path to the config file
    #[clap(
        short,
        long,
        value_parser,
        default_value = "$HOME/.config/wificar.toml"
    )]
    config_path: String,
}

#[tokio::main] // or #[tokio::main]
async fn main() {
    env_logger::init();
    info!(target: "init",
        "web based driver interface v{}",
        env!("CARGO_PKG_VERSION")
    );
    let args: Args = Args::parse();

    let settings: settings::Settings = match Settings::new(args.config_path.clone()) {
        Ok(s) => s,
        Err(e) => {
            error!(target: "config","error loading config: {}", e);
            panic!()
        }
    };

    let driver = Arc::new(Mutex::new(Drivers::new(settings.driver)));

    let api = api::api(driver);

    // View access logs by setting `RUST_LOG=todos`.
    let routes = api.with(warp::log("api"));
    warp::serve(routes).run((settings.ip, settings.port)).await;
}
