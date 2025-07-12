extern crate ddc;
extern crate ddc_macos;

use ddc::Ddc;
use ddc_macos::Monitor;
use std::{env::args, process::ExitCode};

const RELEASE_DATE: &str = "2025-07-12";

fn main() -> ExitCode {
    let mut monitors = match Monitor::enumerate() {
        Ok(vec) => vec,
        Err(_) => return error("Could not enumerate external displays."),
    };
    let first_monitor = match monitors.get_mut(0) {
        Some(mon) => mon,
        None => return error("No external display(s) found."),
    };
    let args = args().collect::<Vec<_>>();
    let args_len = args.len();
    if args_len > 2 {
        return error("Too many arguments.");
    } else if args_len < 2 {
        println!();
        title();
        println!();
        println!("A tiny macOS CLI tool using DDC/CI to control the display's input source");
        println!();
        println!("Usage:");
        println!("   cdis [number]");
        println!();
        println!("Notes:");
        println!("   Use the number argument to change the display's input source.");
        println!("   Given number has to be a decimal VCP60 value between 0-65535.");
        println!("   Check your display manual for the supported DDC VCP60 values.");
        println!();
        println!("Have a look at https://github.com/mbodm/cdis-mac for more information");
        println!();
        let product_name = match first_monitor.product_name() {
            Some(s) => s,
            None => "UNKNOWN".to_string(),
        };
        let vcp_60_value = match first_monitor.get_vcp_feature(0x60) {
            Ok(v) => v.value().to_string(),
            Err(_) => "UNKNOWN".to_string(),
        };
        println!("Display product name: {product_name}");
        println!("Active VCP60 setting: {vcp_60_value}");
        println!();
        println!("Have a nice day.");
        println!();
    } else {
        let number = match args[1].parse() {
            Ok(u) => u,
            Err(_) => return error("Argument has to be a decimal number between 0 and 65535."),
        };
        match first_monitor.set_vcp_feature(0x60, number) {
            Ok(_) => {}
            Err(_) => return error("Could not set input source (DDC VCP60)."),
        };
    }
    return ExitCode::SUCCESS;
}

fn error(msg: &str) -> ExitCode {
    println!();
    title();
    println!();
    println!("Error: {msg}");
    println!();
    return ExitCode::FAILURE;
}

fn title() {
    // No need for some code here, to verify name and version from cargo.toml file,
    // since cargo will show an error, if name or version contains an empty string.
    let app_name = env!("CARGO_PKG_NAME");
    let app_version = env!("CARGO_PKG_VERSION");
    println!("{app_name} {app_version} (by MBODM {RELEASE_DATE})");
}
