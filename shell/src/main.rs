use drivers::drivers::demo::Demo;
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
            /*
             *TODO: make this work
            "skidSteer" => {
                *driver = Some(Drivers::SimpleSkidSteer(SkidSteer::new(
                    select[1].into(),
                    select[2].into(),
                    select[3].into(),
                    select[4].into(),
                )))
            }
            */
            _ => writeln!(io, "that is not a driver")?,
        }
        Ok(())
    });
    shell.new_command_noargs("demo", "run a demo sequence", |io, driver| {
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
        driver
            .as_mut()
            .unwrap()
            .drive(args[0].parse().unwrap(), args[1].parse().unwrap());
        Ok(())
    });

    shell.run_loop(&mut ShellIO::default());
}
