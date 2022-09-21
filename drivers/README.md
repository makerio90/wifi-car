# drivers
combines interfaces to some common rc car setups.
currently supported driver configurations:
- demo ( demo lib, for testing)
- simpleSkidSteer ( standerd motor driver in a skid steer configuration )

more will be added in future updates
## usage

```rust
use drivers::drivers::demo::Demo;
use drivers::drivers::Drivers;
use drivers::Driver;
use std::{thread::sleep, time::Duration};

fn main() {
    let driver = Drivers::Demo(Demo::new())
    driver.enable()
    driver.drive(1.0,0.0)
    driver.drive(1.0, 0.0)?;
    sleep(Duration::from_secs(1));
    driver.drive(-1.0, 0.0)?;
    sleep(Duration::from_secs(1));
    driver.drive(1.0, 1.0)?;
    sleep(Duration::from_secs(1));
    driver.drive(1.0, -1.0)?;
    driver.disable()
}
```
**NOTE**: not intended for independent use, more designd for use with the `shell` crate. support will be given out by request but i will not actavely devolop this crate for use independantly.
