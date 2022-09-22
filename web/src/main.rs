pub mod settings;

use clap::Parser;
use colored::Colorize;
use config::Config;
use drivers::drivers::{demo::Demo, simple_skid_steer::SkidSteer, Drivers};
use drivers::Driver;
use log::{debug, info, warn};
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
    configPath: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();
    info!(
        "Shell based driver interface v{}",
        env!("CARGO_PKG_VERSION")
    );

    info!("loading config file in: {}", args.configPath);
    let settings = Settings::new(args.configPath)?;
    info!("loaded config");
    let mut driver: Option<Drivers> = None;

    // from string to data
    match settings.driver.as_str() {
        "demo" => driver = Some(Drivers::Demo(Demo::new())),
        "skidSteer" => driver = Some(Drivers::SimpleSkidSteer(SkidSteer::new(0, 6, 5, 12))),
        _ => println!("that is not a driver"),
    }

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
    Ok(())
}
