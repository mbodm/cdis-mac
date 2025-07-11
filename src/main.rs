extern crate ddc;
extern crate ddc_macos;

use ddc::Ddc;
use ddc_macos::Monitor;

use std::{env::args, process::ExitCode};

// No need for some code here, to verify name and version from cargo.toml file,
// since cargo will show an error, if name or version contains an empty string.

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const RELEASE_DATE: &str = "2025-07-03";

fn main() -> ExitCode {
    println!();
    println!("{APP_NAME} {APP_VERSION} (by MBODM {RELEASE_DATE})");
    println!();
    println!("A tiny macOS CLI tool using DDC/CI to control the display's input source");
    println!();
    let mut monitors = match Monitor::enumerate() {
        Ok(x) => x,
        Err(_) => return error("Could not enumerate external displays."),
    };
    let first_monitor = match monitors.get_mut(0) {
        Some(x) => x,
        None => return error("No external display(s) found."),
    };
    let args = args().collect::<Vec<_>>();
    if args.len() < 2 {
        let product_name = match first_monitor.product_name() {
            Some(x) => x,
            None => "Unknown".to_string(),
        };
        let vcp_60_value: u16 = match first_monitor.get_vcp_feature(0x60) {
            Ok(vcp) => vcp.value(),
            Err(_) => 0,
        };
        println!("Display product name: {product_name}");
        println!("Active VCP60 setting: {vcp_60_value}");
        println!();
        println!("Usage:");
        println!("   cdis [number]");
        println!();
        println!("Notes:");
        println!("   - Use the number argument to change the display's input source");
        println!("   - Given number has to be a decimal VCP60 value between 0-65535");
        println!("   - Check your display manual for the supported DDC VCP60 values");
        println!();
        println!("Have a look at https://github.com/mbodm/cdis for more information");
        println!();
        return ExitCode::SUCCESS;
    }
    let number = match args[1].parse() {
        Ok(x) => x,
        Err(_) => return error("Error: Argument has to be a decimal number between 0 and 65535."),
    };
    match first_monitor.set_vcp_feature(0x60, number) {
        Ok(_) => println!("New VCP60 setting: {number}"),
        Err(_) => return error("Could not set input source (VCP60 value)."),
    };
    println!();
    println!("Have a nice day.");
    println!();
    return ExitCode::SUCCESS;
}

fn error(msg: &str) -> ExitCode {
    println!("Error: {msg}");
    return ExitCode::FAILURE;
}
