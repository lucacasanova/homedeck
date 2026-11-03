//! Core of a homedeck node.
//!
//! Modules land here as the roadmap advances: library indexing, playback,
//! discovery, pairing and streaming (see `docs/planning/ARCHITECTURE.md`).

/// Product name, as shown to users.
pub const NAME: &str = "homedeck";

/// Version of this build, taken from the workspace manifest.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Human-readable identification of this build, e.g. `homedeck 0.1.0`.
#[must_use]
pub fn build_label() -> String {
    format!("{NAME} {VERSION}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_label_has_name_and_version() {
        let label = build_label();
        assert!(label.starts_with("homedeck "));
        assert!(label.ends_with(VERSION));
    }
}
