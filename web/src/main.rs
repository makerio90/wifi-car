pub mod settings;

use actix_web::{get, web, App, HttpServer, Responder};
use clap::Parser;
use drivers::drivers::{demo::Demo, simple_skid_steer::SkidSteer, Drivers};
use drivers::Driver;
use lazy_static::lazy_static;
use log::{error, info};
use settings::Settings;
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
    static ref DRIVER: Drivers = match SETTINGS.driver.as_str() {
        "demo" => Drivers::Demo(Demo::new()),
        // TODO: make this configurable
        "skidSteer" => Drivers::SimpleSkidSteer(SkidSteer::new(0, 6, 5, 12)),
        d => {
            error!(target:"driver","`{}` is not a driver! quitting...", d);
            panic!()
        }
    };
}

#[get("/api/info")]
async fn driver_info() -> impl Responder {
    format!(
        "using driver {}! \n break: {}",
        SETTINGS.driver,
        DRIVER.has_break()
    )
}
#[actix_web::main] // or #[tokio::main]
async fn main() {
    env_logger::init();
    info!(target: "init",
        "web based driver interface v{}",
        env!("CARGO_PKG_VERSION")
    );

    info!(target: "server", "starting server at http://{}:{}/", &SETTINGS.ip, SETTINGS.port);
    // start the server
    match HttpServer::new(|| App::new().service(driver_info))
        .bind((SETTINGS.ip.clone(), SETTINGS.port))
    {
        Ok(s) => s,
        Err(e) => {
            error!(target: "server","error starting server: {}", e);
            panic!()
        }
    }
    .run()
    .await;
}
