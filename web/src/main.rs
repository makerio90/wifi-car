pub mod settings;

use clap::Parser;
use colored::Colorize;
use drivers::drivers::{demo::Demo, simple_skid_steer::SkidSteer, Drivers};
use drivers::Driver;
use log::{debug, error, info, log_enabled, Level};
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

fn main() {
    env_logger::init();
    let args = Args::parse();
    info!(target: "init",
        "Shell based driver interface v{}",
        env!("CARGO_PKG_VERSION")
    );

    info!(
        target : "config",
        "loading config file in: {}", args.config_path
    );
    let settings = match Settings::new(args.config_path) {
        Ok(s) => s,
        Err(e) => {
            error!(target: "config","error loading config: {}", e);
            std::process::exit(1)
        }
    };
    info!(target : "config", "loaded config");
    let mut driver: Option<Drivers> = None;

    // from string to data
    match settings.driver.as_str() {
        "demo" => driver = Some(Drivers::Demo(Demo::new())),
        // TODO: make this configurable
        "skidSteer" => driver = Some(Drivers::SimpleSkidSteer(SkidSteer::new(0, 6, 5, 12))),
        d => {
            error!("`{}` is not a driver! quitting...", d);
            std::process::exit(1)
        }
    }

    if log_enabled!(Level::Debug) {
        // test
        let d = &mut driver.as_mut().unwrap();
        debug!(
            "enabled: {}",
            if d.is_ready() {
                "yes".green()
            } else {
                "no".red()
            }
        );
        debug!(
            "has break: {}",
            if d.has_break() {
                "yes".green()
            } else {
                "no".red()
            }
        );
        let (steer, drive) = d.is_proportional();
        debug!(
            "proportional drive: {}",
            if drive { "yes".green() } else { "no".red() }
        );
        debug!(
            "proportional steering: {}",
            if steer { "yes".green() } else { "no".red() }
        );
    }
}
