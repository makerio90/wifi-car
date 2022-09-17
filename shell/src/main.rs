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
        "Shell baised driver interface v{}",
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
        d.drive(1.0, 0.0);
        sleep(Duration::from_secs(1));
        d.drive(-1.0, 0.0);
        sleep(Duration::from_secs(1));
        d.drive(1.0, 1.0);
        sleep(Duration::from_secs(1));
        d.drive(1.0, -1.0);
        Ok(())
    });
    shell.new_command("drive", "set the Driver", 2, |io, driver, args| {
        match driver
            .as_mut()
            .unwrap()
            .drive(args[0].parse().unwrap(), args[1].parse().unwrap())
        {
            Err(e) => writeln!(io, "error: {:?}", e)?,
            Ok(_) => writeln!(io, "Ok")?,
        };
        Ok(())
    });
    shell.new_command_noargs("info", "displays driver info", |io, driver| {
        let d = &mut driver.as_mut().unwrap();
        writeln!(io, "enabled: {}", d.is_ready())?;
        writeln!(io, "has break: {}", d.has_break())?;
        let (steer, drive) = d.is_proportional();
        writeln!(io, "proportional drive: {}", drive)?;
        writeln!(io, "proportional steering: {}", steer)?;
        Ok(())
    });
    shell.new_command_noargs("enable", "enables the driver", |io, driver| {
        match driver.as_mut().unwrap().enable() {
            Err(e) => writeln!(io, "error: {:?}", e)?,
            Ok(_) => writeln!(io, "Ok")?,
        };
        Ok(())
    });

    shell.run_loop(&mut ShellIO::default());
}
