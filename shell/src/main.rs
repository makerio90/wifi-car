use colored::Colorize;
use drivers::drivers::demo::Demo;
use drivers::drivers::simple_skid_steer::SkidSteer;
use drivers::drivers::Drivers;
use drivers::Driver;
use shrust::{Shell, ShellIO};
use std::env;
use std::io::Write;
use std::{thread::sleep, time::Duration};

fn main() {
    println!(
        "Shell based driver interface v{}",
        env!("CARGO_PKG_VERSION")
    );
    let driver: Option<Drivers> = None;

    let mut shell = Shell::new(driver);
    shell.new_command("set", "set the driver", 1, |io, driver, select| {
        match select[0] {
            "demo" => *driver = Some(Drivers::Demo(Demo::new())),
            // TODO: dynamicly set pin numbers
            "skidSteer" => *driver = Some(Drivers::SimpleSkidSteer(SkidSteer::new(0, 6, 5, 12))),
            _ => writeln!(io, "that is not a driver")?,
        }
        Ok(())
    });
    shell.new_command_noargs("demo", "run a demo sequence", |_, driver| {
        let d = &mut driver.as_mut().unwrap();
        d.drive(1.0, 0.0)?;
        sleep(Duration::from_secs(1));
        d.drive(-1.0, 0.0)?;
        sleep(Duration::from_secs(1));
        d.drive(1.0, 1.0)?;
        sleep(Duration::from_secs(1));
        d.drive(1.0, -1.0)?;
        Ok(())
    });
    shell.new_command("drive", "set the Driver", 2, |_, driver, args| {
        driver
            .as_mut()
            .unwrap()
            .drive(args[0].parse().unwrap(), args[1].parse().unwrap())?;
        Ok(())
    });
    shell.new_command_noargs("info", "displays driver info", |io, driver| {
        let d = &mut driver.as_mut().unwrap();
        writeln!(
            io,
            "enabled: {}",
            if d.is_ready() {
                "yes".green()
            } else {
                "no".red()
            }
        )?;
        writeln!(
            io,
            "has break: {}",
            if d.has_break() {
                "yes".green()
            } else {
                "no".red()
            }
        )?;
        let (steer, drive) = d.is_proportional();
        writeln!(
            io,
            "proportional drive: {}",
            if drive { "yes".green() } else { "no".red() }
        )?;
        writeln!(
            io,
            "proportional steering: {}",
            if steer { "yes".green() } else { "no".red() }
        )?;
        Ok(())
    });
    shell.new_command_noargs("enable", "enables the driver", |_, driver| {
        driver.as_mut().unwrap().enable()?;
        Ok(())
    });
    shell.new_command_noargs("disable", "disables the driver", |_, driver| {
        driver.as_mut().unwrap().disable()?;
        Ok(())
    });
    shell.new_command_noargs("estop", "estop the driver", |_, driver| {
        driver.as_mut().unwrap().estop()?;
        Ok(())
    });

    shell.run_loop(&mut ShellIO::default());
}
