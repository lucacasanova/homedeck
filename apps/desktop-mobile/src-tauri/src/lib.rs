//! homedeck shell: wires the shared UI to the node core.

use tauri_plugin_log::{RotationStrategy, Target, TargetKind};

/// Returns the build label (e.g. `homedeck 0.1.0`) to the UI.
#[tauri::command]
fn app_info() -> String {
    homedeck_core::build_label()
}

/// Rolling log files under the platform log dir
/// (Linux: `~/.local/share/io.github.lucacasanova.homedeck/logs/`).
fn log_plugin<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri_plugin_log::Builder::new()
        .targets([
            Target::new(TargetKind::Stdout),
            Target::new(TargetKind::LogDir {
                file_name: Some("homedeck".into()),
            }),
        ])
        .level(log::LevelFilter::Info)
        .max_file_size(5_000_000)
        .rotation_strategy(RotationStrategy::KeepSome(5))
        .build()
}

/// Starts the app. Shared by the desktop binary and the mobile entry point.
///
/// Desktop opens as a normal window; size, position and fullscreen (F11) are
/// remembered between launches. Mobile and TV run fullscreen by platform default.
///
/// # Panics
/// Panics if the Tauri runtime fails to start (e.g. no display available).
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default().plugin(log_plugin());

    #[cfg(desktop)]
    let builder = builder.plugin(tauri_plugin_window_state::Builder::default().build());

    #[allow(clippy::expect_used)]
    builder
        .setup(|_app| {
            log::info!(
                "{} starting on {} {}",
                homedeck_core::build_label(),
                std::env::consts::OS,
                std::env::consts::ARCH
            );
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![app_info])
        .run(tauri::generate_context!())
        .expect("failed to start homedeck");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_info_comes_from_core() {
        assert_eq!(app_info(), homedeck_core::build_label());
    }
}
