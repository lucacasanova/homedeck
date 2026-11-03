//! Desktop entry point. Keep logic in `lib.rs` so mobile builds share it.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    homedeck_app_lib::run();
}
