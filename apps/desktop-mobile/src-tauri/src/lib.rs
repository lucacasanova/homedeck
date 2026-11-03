//! homedeck shell: wires the shared UI to the node core.

/// Returns the build label (e.g. `homedeck 0.1.0`) to the UI.
#[tauri::command]
fn app_info() -> String {
    homedeck_core::build_label()
}

/// Starts the app. Shared by the desktop binary and the mobile entry point.
///
/// # Panics
/// Panics if the Tauri runtime fails to start (e.g. no display available).
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[allow(clippy::expect_used)]
    tauri::Builder::default()
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
