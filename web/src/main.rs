pub mod server;
pub mod settings;

use clap::Parser;
use drivers::drivers::{demo::Demo, simple_skid_steer::SkidSteer, Drivers};
use drivers::Driver;
use lazy_static::lazy_static;
use log::{error, info, trace};
use server::end;
use settings::Settings;
use std::sync::Mutex;
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

lazy_static! {
    static ref ARGS: Args = Args::parse();
    static ref SETTINGS: settings::Settings = match Settings::new(ARGS.config_path.clone()) {
        Ok(s) => s,
        Err(e) => {
            error!(target: "config","error loading config: {}", e);
            panic!()
        }
    };
    static ref DRIVER: Mutex<Drivers> = Mutex::new(match SETTINGS.driver.as_str() {
        "demo" => Drivers::Demo(Demo::new()),
        // TODO: make this configurable
        "skidSteer" => Drivers::SimpleSkidSteer(SkidSteer::new(0, 6, 5, 12)),
        d => {
            error!(target:"driver","`{}` is not a driver! quitting...", d);
            panic!()
        }
    });
}

#[tokio::main] // or #[tokio::main]
async fn main() {
    env_logger::init();
    info!(target: "init",
        "web based driver interface v{}",
        env!("CARGO_PKG_VERSION")
    );

    warp::serve(end()).run(([127, 0, 0, 1], 3030)).await;
}
