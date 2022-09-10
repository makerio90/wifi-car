use drivers::Drivers;
use std::env;

fn main() {
    println!(
        "Shell baised driver interface v{}",
        env!("CARGO_PKG_VERSION")
    );
    println!("Hello, world!");
}
