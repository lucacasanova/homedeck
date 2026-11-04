//! Environment report for crash reports and `homedeck --diagnostics`.
//!
//! Only machine facts that help explain graphics/startup failures. Everything
//! that goes into a report passes through [`redact`]: home directory, user
//! name, host name, runtime dir and personal file paths are removed.

use std::fmt::Write as _;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Bundle identifier, used for per-user data folders.
pub const APP_ID: &str = "io.github.lucacasanova.homedeck";

/// The user's home directory: `$HOME`, else the passwd entry. `None` means we
/// must not write anything (never fall back to a shared dir like `/tmp`).
#[must_use]
pub fn home() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .filter(|h| !h.is_empty())
        .map(PathBuf::from)
        .or_else(|| {
            nix::unistd::User::from_uid(nix::unistd::getuid())
                .ok()
                .flatten()
                .map(|u| u.dir)
        })
        .filter(|p| p.is_absolute() && p != Path::new("/"))
}

/// `~/.local/share/<APP_ID>` (honours `XDG_DATA_HOME`).
#[must_use]
pub fn data_dir() -> Option<PathBuf> {
    xdg_dir("XDG_DATA_HOME", ".local/share").map(|d| d.join(APP_ID))
}

/// `~/.config/<APP_ID>` (honours `XDG_CONFIG_HOME`).
#[must_use]
pub fn config_dir() -> Option<PathBuf> {
    xdg_dir("XDG_CONFIG_HOME", ".config").map(|d| d.join(APP_ID))
}

fn xdg_dir(var: &str, fallback: &str) -> Option<PathBuf> {
    std::env::var_os(var)
        .map(PathBuf::from)
        .filter(|v| v.is_absolute())
        .or_else(|| home().map(|h| h.join(fallback)))
}

/// Writes a private file (0600) inside a private directory (0700).
///
/// # Errors
/// Returns the I/O error if the directory or file cannot be created.
pub fn write_private(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    use std::os::unix::fs::{DirBuilderExt as _, OpenOptionsExt as _};
    if let Some(dir) = path.parent() {
        std::fs::DirBuilder::new()
            .recursive(true)
            .mode(0o700)
            .create(dir)?;
    }
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(path)?;
    f.write_all(bytes)
}

fn user_name() -> Option<String> {
    std::env::var("USER")
        .ok()
        .or_else(|| std::env::var("LOGNAME").ok())
        .or_else(|| {
            nix::unistd::User::from_uid(nix::unistd::getuid())
                .ok()
                .flatten()
                .map(|u| u.name)
        })
        .filter(|n| n.len() >= 3)
}

fn host_name() -> Option<String> {
    nix::unistd::gethostname()
        .ok()
        .and_then(|h| h.into_string().ok())
        .filter(|h| h.len() >= 3 && h != "localhost")
}

/// Directories whose contents are personal (media, mounts, homes).
const PERSONAL_PREFIXES: &[&str] = &[
    "/home/",
    "/var/home/",
    "/media/",
    "/run/media/",
    "/mnt/",
    "~/",
];

fn is_path_end(c: char) -> bool {
    c.is_whitespace()
        || matches!(
            c,
            '"' | '\'' | '`' | '(' | ')' | '[' | ']' | '<' | '>' | ',' | ';'
        )
}

/// Replaces personal paths with `<caminho>` (keeping the extension, which is
/// useful for codec problems). System paths like `/usr/lib/...` are kept.
fn redact_paths(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    'outer: while !rest.is_empty() {
        for prefix in PERSONAL_PREFIXES {
            if let Some(pos) = rest.find(prefix) {
                // Only treat it as a path if it starts a token.
                let starts_token =
                    pos == 0 || rest[..pos].chars().next_back().is_some_and(is_path_end);
                if starts_token {
                    let end = rest[pos..]
                        .find(is_path_end)
                        .map_or(rest.len(), |e| pos + e);
                    let path = &rest[pos..end];
                    out.push_str(&rest[..pos]);
                    out.push_str("<caminho>");
                    if let Some(ext) = Path::new(path)
                        .extension()
                        .and_then(|e| e.to_str())
                        .filter(|e| e.len() <= 5)
                    {
                        out.push('.');
                        out.push_str(ext);
                    }
                    rest = &rest[end..];
                    continue 'outer;
                }
            }
        }
        out.push_str(rest);
        break;
    }
    out
}

fn replace_word(text: &str, word: &str, with: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    while let Some(found) = text[i..].find(word) {
        let start = i + found;
        let end = start + word.len();
        let before = text[..start].chars().next_back();
        let after = text[end..].chars().next();
        let boundary = |c: Option<char>| c.is_none_or(|c| !c.is_alphanumeric() && c != '_');
        out.push_str(&text[i..start]);
        out.push_str(if boundary(before) && boundary(after) {
            with
        } else {
            word
        });
        i = end;
    }
    out.push_str(&text[i..]);
    out
}

/// Removes personal data from any text that goes into a report.
#[must_use]
pub fn redact(text: &str) -> String {
    let mut s = text.to_owned();
    if let Some(home) = home() {
        let mut homes = vec![home.to_string_lossy().into_owned()];
        if let Ok(real) = std::fs::canonicalize(&home) {
            homes.push(real.to_string_lossy().into_owned());
        }
        for h in homes {
            s = s.replace(&h, "~");
        }
    }
    s = s.replace(
        &format!("/run/user/{}", nix::unistd::getuid()),
        "/run/user/<uid>",
    );
    s = redact_paths(&s);
    if let Some(user) = user_name() {
        s = replace_word(&s, &user, "<usuário>");
    }
    if let Some(host) = host_name() {
        s = replace_word(&s, &host, "<host>");
    }
    s
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

/// Last `n` lines of the app log, redacted.
#[must_use]
pub fn log_tail(n: usize) -> String {
    let Some(dir) = data_dir() else {
        return String::new();
    };
    let text = std::fs::read_to_string(dir.join("logs/homedeck.log")).unwrap_or_default();
    let lines: Vec<&str> = text.lines().collect();
    redact(&lines[lines.len().saturating_sub(n)..].join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redact_removes_home() {
        let Some(home) = home() else { return };
        let out = redact(&format!("{}/Videos/ferias.mkv", home.display()));
        assert!(!out.contains(&*home.to_string_lossy()));
        assert!(
            out.starts_with('~') || out.starts_with("<caminho>"),
            "{out}"
        );
    }

    #[test]
    fn redact_hides_personal_paths_but_keeps_system_paths() {
        let out = redact(
            "abrindo /run/media/fulano/HD/Filmes/x.mkv e /mnt/nas/a b.mp4 via /usr/lib/libEGL.so.1",
        );
        assert!(!out.contains("fulano"));
        assert!(!out.contains("Filmes"));
        assert!(out.contains("<caminho>.mkv"));
        assert!(out.contains("/usr/lib/libEGL.so.1"));
    }

    #[test]
    fn redact_hides_user_and_host_as_words_only() {
        let s = replace_word("user bob logged as bob-x; bobby stays", "bob", "<usuário>");
        assert_eq!(s, "user <usuário> logged as <usuário>-x; bobby stays");
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
        if let (Some(d), Some(c)) = (data_dir(), config_dir()) {
            assert!(d.ends_with(APP_ID));
            assert!(c.ends_with(APP_ID));
        }
    }

    #[test]
    fn private_files_are_0600_in_0700_dirs() {
        use std::os::unix::fs::PermissionsExt as _;
        let dir = std::env::temp_dir().join(format!("hd-test-{}", std::process::id()));
        let file = dir.join("sub/report.txt");
        write_private(&file, b"x").expect("write");
        let fmode = std::fs::metadata(&file).expect("meta").permissions().mode() & 0o777;
        let dmode = std::fs::metadata(file.parent().expect("parent"))
            .expect("meta")
            .permissions()
            .mode()
            & 0o777;
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(fmode, 0o600);
        assert_eq!(dmode, 0o700);
    }
}
