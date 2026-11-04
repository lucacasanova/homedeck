//! Desktop entry point. Keep logic in `lib.rs` so mobile builds share it.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // On Linux the binary first acts as a startup supervisor (ADR 0010):
    // it runs the UI as a child and falls back to compatibility modes on
    // graphics crashes. `None` means "this process is the UI".
    #[cfg(target_os = "linux")]
    if let Some(code) = homedeck_app_lib::launcher::supervise() {
        std::process::exit(code);
    }
    homedeck_app_lib::run();
}
