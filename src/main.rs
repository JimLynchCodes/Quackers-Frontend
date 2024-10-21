// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;
use quackers_beta::AppPlugin;

// use web_sys::console;
// use log::Level;
use wasm_logger;

fn main() -> AppExit {
    wasm_logger::init(wasm_logger::Config::default());

    // // Logging
    log::info!("Some info");
    log::trace!("Some trace");
    log::error!("Error message");

    // console::log_1(&"Hello from Bevy and WebAssembly!".into());
    App::new().add_plugins(AppPlugin).run()
}
