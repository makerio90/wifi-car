use drivers::drivers;
use shrust::{Shell, ShellIO};
use std::env;
use std::io::Write;

fn main() {
    println!(
        "Shell baised driver interface v{}",
        env!("CARGO_PKG_VERSION")
    );
    let mut driver: Option<Drivers> = None;

    let mut shell = Shell::new(driver);
    shell.new_command("set", "set the driver", 1, |io, driver, select| ok(()));

    shell.run_loop(&mut ShellIO::default());
}
