//! Environment report for crash reports and `homedeck --diagnostics`.
//!
//! Only machine facts that help explain graphics/startup failures. The home
//! directory is replaced by `~` so the report never carries the user name.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Bundle identifier, used for per-user data folders.
pub const APP_ID: &str = "io.github.lucacasanova.homedeck";

/// `~/.local/share/<APP_ID>` (honours `XDG_DATA_HOME`).
#[must_use]
pub fn data_dir() -> PathBuf {
    xdg_dir("XDG_DATA_HOME", ".local/share").join(APP_ID)
}

/// `~/.config/<APP_ID>` (honours `XDG_CONFIG_HOME`).
#[must_use]
pub fn config_dir() -> PathBuf {
    xdg_dir("XDG_CONFIG_HOME", ".config").join(APP_ID)
}

fn xdg_dir(var: &str, fallback: &str) -> PathBuf {
    std::env::var_os(var)
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| home().join(fallback))
}

fn home() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/tmp"))
}

/// Replaces the home directory with `~` in any text that goes into a report.
#[must_use]
pub fn redact(text: &str) -> String {
    let home = home();
    let home = home.to_string_lossy();
    if home.len() > 1 {
        text.replace(home.as_ref(), "~")
    } else {
        text.to_owned()
    }
}

fn read_trim(path: &Path) -> Option<String> {
    std::fs::read_to_string(path)
        .ok()
        .map(|s| s.trim().to_owned())
}

fn os_release() -> String {
    read_trim(Path::new("/etc/os-release"))
        .and_then(|s| {
            s.lines()
                .find_map(|l| l.strip_prefix("PRETTY_NAME="))
                .map(|v| v.trim_matches('"').to_owned())
        })
        .unwrap_or_else(|| "desconhecido".into())
}

fn gpus() -> Vec<String> {
    let lspci = Command::new("lspci").arg("-nn").output().ok();
    let from_lspci: Vec<String> = lspci
        .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
        .unwrap_or_default()
        .lines()
        .filter(|l| l.contains("VGA") || l.contains("3D") || l.contains("Display"))
        .map(|l| l.split_once(": ").map_or(l, |(_, v)| v).to_owned())
        .collect();
    if !from_lspci.is_empty() {
        return from_lspci;
    }
    // Fallback without pciutils: vendor ids from sysfs.
    std::fs::read_dir("/sys/class/drm")
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|e| read_trim(&e.path().join("device/vendor")))
        .collect()
}

fn egl_vendors() -> Vec<String> {
    let mut v: Vec<String> = ["/usr/share/glvnd/egl_vendor.d", "/etc/glvnd/egl_vendor.d"]
        .iter()
        .filter_map(|d| std::fs::read_dir(d).ok())
        .flatten()
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    v.sort();
    v.dedup();
    v
}

/// Path of the Mesa EGL vendor file, if installed (used by the `mesa-egl` mode).
#[must_use]
pub fn mesa_egl_vendor_file() -> Option<PathBuf> {
    ["/usr/share/glvnd/egl_vendor.d", "/etc/glvnd/egl_vendor.d"]
        .iter()
        .map(|d| Path::new(d).join("50_mesa.json"))
        .find(|p| p.exists())
}

fn env_or(var: &str, default: &str) -> String {
    std::env::var(var).unwrap_or_else(|_| default.to_owned())
}

/// Machine summary used in every report.
#[must_use]
pub fn environment_report() -> String {
    let mut r = String::new();
    let _ = writeln!(r, "app: {}", homedeck_core::build_label());
    let _ = writeln!(
        r,
        "pacote: {}",
        if std::env::var_os("APPIMAGE").is_some() {
            "AppImage"
        } else {
            "nativo"
        }
    );
    let _ = writeln!(r, "sistema: {}", os_release());
    let _ = writeln!(
        r,
        "kernel: {}",
        read_trim(Path::new("/proc/sys/kernel/osrelease")).unwrap_or_default()
    );
    let _ = writeln!(r, "arquitetura: {}", std::env::consts::ARCH);
    let _ = writeln!(r, "sessão: {}", env_or("XDG_SESSION_TYPE", "desconhecida"));
    let _ = writeln!(
        r,
        "desktop: {}",
        env_or("XDG_CURRENT_DESKTOP", "desconhecido")
    );
    for gpu in gpus() {
        let _ = writeln!(r, "gpu: {gpu}");
    }
    if let Some(nv) = read_trim(Path::new("/proc/driver/nvidia/version")) {
        let first = nv.lines().next().unwrap_or_default().to_owned();
        let _ = writeln!(r, "nvidia: {first}");
    }
    let _ = writeln!(r, "egl vendors: {}", egl_vendors().join(", "));
    redact(&r)
}

/// Last `n` lines of the app log, if any.
#[must_use]
pub fn log_tail(n: usize) -> String {
    let path = data_dir().join("logs/homedeck.log");
    let text = std::fs::read_to_string(path).unwrap_or_default();
    let lines: Vec<&str> = text.lines().collect();
    redact(&lines[lines.len().saturating_sub(n)..].join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redact_removes_home() {
        let home = home();
        let input = format!("{}/Videos/ferias.mkv", home.display());
        let out = redact(&input);
        assert!(!out.contains(&*home.to_string_lossy()));
        assert!(out.starts_with("~/"));
    }

    #[test]
    fn report_has_core_fields() {
        let r = environment_report();
        for key in ["app:", "sistema:", "sessão:", "arquitetura:"] {
            assert!(r.contains(key), "missing {key} in\n{r}");
        }
    }

    #[test]
    fn dirs_use_app_id() {
        assert!(data_dir().ends_with(APP_ID));
        assert!(config_dir().ends_with(APP_ID));
    }
}
