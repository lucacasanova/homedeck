//! homedeck shell: wires the shared UI to the node core.

pub mod diagnostics;
#[cfg(target_os = "linux")]
pub mod launcher;

use tauri_plugin_log::{RotationStrategy, Target, TargetKind};

/// Tells the startup supervisor (ADR 0010) that the UI loaded, so a later exit
/// is not treated as a graphics/startup failure.
fn signal_ready() {
    if let Some(path) = std::env::var_os("HOMEDECK_READY_FILE") {
        let _ = std::fs::write(path, b"ready");
    }
}

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
    // Test hook for the supervisor: crash in the listed compatibility modes to
    // simulate a graphics driver failure (`HOMEDECK_TEST_CRASH_MODES=default,no-dmabuf`).
    if let (Ok(list), Ok(mode)) = (
        std::env::var("HOMEDECK_TEST_CRASH_MODES"),
        std::env::var("HOMEDECK_COMPAT"),
    ) && list.split(',').any(|m| m.trim() == mode)
    {
        eprintln!("simulated crash in compat mode `{mode}`");
        std::process::abort();
    }

    let builder = tauri::Builder::default().plugin(log_plugin());

    #[cfg(desktop)]
    let builder = builder.plugin(tauri_plugin_window_state::Builder::default().build());

    #[allow(clippy::expect_used)]
    builder
        .setup(|_app| {
            log::info!(
                "{} starting on {} {} (compat mode: {})",
                homedeck_core::build_label(),
                std::env::consts::OS,
                std::env::consts::ARCH,
                std::env::var("HOMEDECK_COMPAT").unwrap_or_else(|_| "none".into())
            );
            Ok(())
        })
        .on_page_load(|_webview, payload| {
            if payload.event() == tauri::webview::PageLoadEvent::Finished {
                signal_ready();
            }
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
