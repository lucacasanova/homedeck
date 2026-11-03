//! Startup supervisor (ADR 0010).
//!
//! The binary first runs as a light supervisor (no WebKit, no GPU). It starts
//! the real UI as a child process and waits for a "ready" marker written when
//! the page finishes loading. If the child dies before that — typically a
//! graphics driver crash — it writes a local report and retries with the next
//! compatibility mode. The mode that works is remembered.

use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::diagnostics::{config_dir, data_dir, environment_report, log_tail, redact};

/// Env var set on the child so it does not supervise itself.
pub const CHILD_ENV: &str = "HOMEDECK_CHILD";
/// Env var with the path the child touches once the UI has loaded.
pub const READY_ENV: &str = "HOMEDECK_READY_FILE";
/// Env var with the compatibility mode in use (for logs and the UI).
pub const MODE_ENV: &str = "HOMEDECK_COMPAT";

/// How long a child may take to show the UI before we stop waiting for "ready"
/// (it keeps running; we just stop treating an exit as a startup failure).
const READY_TIMEOUT: Duration = Duration::from_secs(45);

/// A compatibility mode: a name and the environment it adds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mode {
    /// Stable identifier, saved to disk and accepted by `--compat`.
    pub name: &'static str,
    /// Extra environment variables for the child.
    pub env: Vec<(&'static str, String)>,
}

/// All modes, from least to most intrusive.
#[must_use]
pub fn modes() -> Vec<Mode> {
    let mut list = vec![
        Mode {
            name: "default",
            env: vec![],
        },
        Mode {
            name: "no-dmabuf",
            env: vec![("WEBKIT_DISABLE_DMABUF_RENDERER", "1".into())],
        },
    ];
    if let Some(mesa) = crate::diagnostics::mesa_egl_vendor_file() {
        list.push(Mode {
            name: "mesa-egl",
            env: vec![
                (
                    "__EGL_VENDOR_LIBRARY_FILENAMES",
                    mesa.to_string_lossy().into_owned(),
                ),
                ("WEBKIT_DISABLE_DMABUF_RENDERER", "1".into()),
            ],
        });
    }
    list.push(Mode {
        name: "x11",
        env: vec![
            ("GDK_BACKEND", "x11".into()),
            ("WEBKIT_DISABLE_DMABUF_RENDERER", "1".into()),
        ],
    });
    list.push(Mode {
        name: "software",
        env: vec![
            ("LIBGL_ALWAYS_SOFTWARE", "1".into()),
            ("WEBKIT_DISABLE_COMPOSITING_MODE", "1".into()),
            ("WEBKIT_DISABLE_DMABUF_RENDERER", "1".into()),
        ],
    });
    list
}

/// Puts `preferred` first (if it exists), keeping the rest in order.
#[must_use]
pub fn ordered(mut list: Vec<Mode>, preferred: Option<&str>) -> Vec<Mode> {
    if let Some(i) = preferred.and_then(|name| list.iter().position(|m| m.name == name)) {
        let m = list.remove(i);
        list.insert(0, m);
    }
    list
}

fn saved_mode_file() -> PathBuf {
    config_dir().join("compat-mode")
}

fn load_saved_mode() -> Option<String> {
    std::fs::read_to_string(saved_mode_file())
        .ok()
        .map(|s| s.trim().to_owned())
        .filter(|s| !s.is_empty())
}

fn save_mode(name: &str) {
    let file = saved_mode_file();
    if let Some(dir) = file.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let _ = std::fs::write(file, name);
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

/// Human description of how a process ended.
#[must_use]
pub fn describe_exit(status: ExitStatus) -> String {
    use std::os::unix::process::ExitStatusExt as _;
    match (status.code(), status.signal()) {
        (Some(code), _) => format!("saiu com código {code}"),
        (None, Some(11)) => "falha de segmentação (SIGSEGV)".into(),
        (None, Some(6)) => "abortado (SIGABRT)".into(),
        (None, Some(sig)) => format!("encerrado pelo sinal {sig}"),
        (None, None) => "encerrado".into(),
    }
}

/// One failed attempt, as recorded in the report.
#[derive(Debug, Clone)]
pub struct Attempt {
    /// Mode name.
    pub mode: &'static str,
    /// How the child ended.
    pub outcome: String,
    /// Last stderr lines of the child.
    pub stderr_tail: Vec<String>,
}

/// Builds the plain-text crash report.
#[must_use]
pub fn render_report(attempts: &[Attempt], recovered_with: Option<&str>) -> String {
    let mut r = String::from("# homedeck — relatório de inicialização\n\n");
    r.push_str(&environment_report());
    r.push('\n');
    match recovered_with {
        Some(mode) => r.push_str(&format!(
            "resultado: abriu no modo de compatibilidade `{mode}`\n"
        )),
        None => r.push_str("resultado: não abriu em nenhum modo\n"),
    }
    for a in attempts {
        r.push_str(&format!("\n## modo `{}`: {}\n", a.mode, a.outcome));
        if !a.stderr_tail.is_empty() {
            r.push_str("```\n");
            r.push_str(&redact(&a.stderr_tail.join("\n")));
            r.push_str("\n```\n");
        }
    }
    let tail = log_tail(40);
    if !tail.is_empty() {
        r.push_str("\n## fim do log do app\n```\n");
        r.push_str(&tail);
        r.push_str("\n```\n");
    }
    r
}

fn write_report(text: &str) -> Option<PathBuf> {
    let dir = data_dir().join("crash");
    std::fs::create_dir_all(&dir).ok()?;
    let path = dir.join(format!("{}.txt", now_secs()));
    std::fs::write(&path, text).ok()?;
    // Lets the UI offer to view/send the report on the next successful start (#128).
    let _ = std::fs::write(dir.join("pending"), path.to_string_lossy().as_bytes());
    Some(path)
}

enum ChildResult {
    /// UI became ready; carries the final exit status.
    Ran(ExitStatus),
    /// Died before the UI was ready.
    Failed(Attempt),
}

/// Set when the supervisor receives SIGTERM/SIGINT/SIGHUP; the child is then
/// asked to stop too, so no UI window is ever left orphaned.
fn termination_flag() -> Arc<std::sync::atomic::AtomicBool> {
    use signal_hook::consts::{SIGHUP, SIGINT, SIGTERM};
    let flag = Arc::new(std::sync::atomic::AtomicBool::new(false));
    for sig in [SIGTERM, SIGINT, SIGHUP] {
        let _ = signal_hook::flag::register(sig, Arc::clone(&flag));
    }
    flag
}

fn run_child(
    mode: &Mode,
    args: &[String],
    stop: &std::sync::atomic::AtomicBool,
    on_ready: &mut dyn FnMut(),
) -> std::io::Result<ChildResult> {
    let exe = std::env::current_exe()?;
    let ready = std::env::temp_dir().join(format!(
        "homedeck-ready-{}-{}",
        std::process::id(),
        mode.name
    ));
    let _ = std::fs::remove_file(&ready);

    let mut cmd = Command::new(exe);
    cmd.args(args)
        .env(CHILD_ENV, "1")
        .env(READY_ENV, &ready)
        .env(MODE_ENV, mode.name)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::piped());
    for (k, v) in &mode.env {
        cmd.env(k, v);
    }
    let mut child = cmd.spawn()?;

    // Mirror stderr to our own stderr while keeping the last lines for the report.
    let tail: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(VecDeque::new()));
    let reader = child.stderr.take().map(|err| {
        let tail = Arc::clone(&tail);
        std::thread::spawn(move || {
            for line in BufReader::new(err).lines().map_while(Result::ok) {
                let _ = writeln!(std::io::stderr(), "{line}");
                if let Ok(mut t) = tail.lock() {
                    if t.len() == 60 {
                        t.pop_front();
                    }
                    t.push_back(line);
                }
            }
        })
    });

    let started = Instant::now();
    let mut became_ready = false;
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if stop.load(std::sync::atomic::Ordering::Relaxed) {
            // Forward the shutdown request, then reap the child.
            let _ = Command::new("kill")
                .args(["-TERM", &child.id().to_string()])
                .status();
            break child.wait()?;
        }
        if !became_ready && (ready.exists() || started.elapsed() > READY_TIMEOUT) {
            became_ready = true;
            log_line(&format!("modo `{}` abriu a interface", mode.name));
            // Persist the outcome now: the supervisor may be killed later.
            on_ready();
        }
        std::thread::sleep(Duration::from_millis(100));
    };
    if let Some(r) = reader {
        let _ = r.join();
    }
    let _ = std::fs::remove_file(&ready);

    if became_ready || ready.exists() {
        return Ok(ChildResult::Ran(status));
    }
    let stderr_tail = tail
        .lock()
        .map(|t| t.iter().cloned().collect())
        .unwrap_or_default();
    Ok(ChildResult::Failed(Attempt {
        mode: mode.name,
        outcome: describe_exit(status),
        stderr_tail,
    }))
}

fn log_line(msg: &str) {
    let _ = writeln!(std::io::stderr(), "[homedeck] {msg}");
}

/// Shown only when no mode works: a native GTK dialog (no WebKit, no GPU).
fn show_fatal_dialog(report: Option<&Path>, text: &str) {
    use rfd::{MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};
    let where_ = report.map_or_else(
        || "(não foi possível salvar)".to_owned(),
        |p| redact(&p.to_string_lossy()),
    );
    let result = MessageDialog::new()
        .set_level(MessageLevel::Error)
        .set_title("homedeck não conseguiu abrir")
        .set_description(format!(
            "O homedeck tentou vários modos de compatibilidade gráfica e nenhum funcionou neste computador.\n\n\
             Um relatório foi salvo em:\n{where_}\n\n\
             Ele não sai do seu computador, a menos que você escolha reportar."
        ))
        .set_buttons(MessageButtons::YesNoCancelCustom(
            "Abrir pasta".into(),
            "Reportar no GitHub".into(),
            "Fechar".into(),
        ))
        .show();
    match result {
        MessageDialogResult::Custom(c) if c == "Abrir pasta" => {
            if let Some(dir) = report.and_then(Path::parent) {
                let _ = Command::new("xdg-open").arg(dir).spawn();
            }
        }
        MessageDialogResult::Custom(c) if c == "Reportar no GitHub" => {
            let _ = Command::new("xdg-open").arg(github_issue_url(text)).spawn();
        }
        _ => {}
    }
}

/// Prefilled GitHub issue; the user reviews everything before submitting.
#[must_use]
pub fn github_issue_url(report: &str) -> String {
    // Keep the URL under common length limits.
    let body: String = report.chars().take(6000).collect();
    let mut url = String::from(
        "https://github.com/lucacasanova/homedeck/issues/new?labels=type%3Abug%2Cplatform%3Alinux&title=",
    );
    url.push_str(&percent_encode(
        "O homedeck não abre (falha de inicialização)",
    ));
    url.push_str("&body=");
    url.push_str(&percent_encode(&body));
    url
}

fn percent_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 3);
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// Removes `--compat <mode>` so the child receives only app arguments.
#[must_use]
pub fn strip_compat_flag(args: &[String]) -> Vec<String> {
    let mut out = Vec::with_capacity(args.len());
    let mut skip_next = false;
    for a in args {
        if skip_next {
            skip_next = false;
        } else if a == "--compat" {
            skip_next = true;
        } else {
            out.push(a.clone());
        }
    }
    out
}

/// Entry point for `main`. Returns `Some(exit_code)` when the supervisor (or a
/// CLI flag) handled the run, or `None` when this process should start the UI.
#[must_use]
pub fn supervise() -> Option<i32> {
    if std::env::var_os(CHILD_ENV).is_some() {
        return None;
    }
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--no-guard") {
        return None;
    }
    if args.iter().any(|a| a == "--diagnostics") {
        print!("{}", environment_report());
        return Some(0);
    }
    let forced = args
        .iter()
        .position(|a| a == "--compat")
        .and_then(|i| args.get(i + 1))
        .cloned();
    let child_args = strip_compat_flag(&args);

    let saved = load_saved_mode();
    let list = match &forced {
        Some(name) => modes()
            .into_iter()
            .filter(|m| m.name == name.as_str())
            .collect(),
        None => ordered(modes(), saved.as_deref()),
    };
    if list.is_empty() {
        log_line(
            "modo de compatibilidade desconhecido; use default, no-dmabuf, mesa-egl, x11 ou software",
        );
        return Some(2);
    }

    let stop = termination_flag();
    let mut attempts: Vec<Attempt> = Vec::new();
    for mode in &list {
        let mut on_ready = || {
            if !attempts.is_empty() {
                let text = render_report(&attempts, Some(mode.name));
                let path = write_report(&text);
                log_line(&format!(
                    "recuperado no modo `{}`; relatório em {}",
                    mode.name,
                    path.map(|p| redact(&p.to_string_lossy()))
                        .unwrap_or_default()
                ));
            }
            if forced.is_none() && saved.as_deref() != Some(mode.name) {
                save_mode(mode.name);
            }
        };
        match run_child(mode, &child_args, &stop, &mut on_ready) {
            Ok(ChildResult::Ran(status)) => {
                return Some(status.code().unwrap_or(1));
            }
            Ok(ChildResult::Failed(_)) if stop.load(std::sync::atomic::Ordering::Relaxed) => {
                return Some(1);
            }
            Ok(ChildResult::Failed(attempt)) => {
                log_line(&format!(
                    "modo `{}` falhou: {}",
                    attempt.mode, attempt.outcome
                ));
                attempts.push(attempt);
            }
            Err(e) => {
                log_line(&format!("não foi possível iniciar a interface: {e}"));
                return Some(1);
            }
        }
    }

    let text = render_report(&attempts, None);
    let path = write_report(&text);
    show_fatal_dialog(path.as_deref(), &text);
    Some(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_mode_comes_first_and_names_are_unique() {
        let list = modes();
        assert_eq!(list.first().map(|m| m.name), Some("default"));
        let mut names: Vec<_> = list.iter().map(|m| m.name).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), list.len());
    }

    #[test]
    fn saved_mode_is_tried_first() {
        let list = ordered(modes(), Some("x11"));
        assert_eq!(list.first().map(|m| m.name), Some("x11"));
        assert_eq!(list.len(), modes().len());
        let unknown = ordered(modes(), Some("nope"));
        assert_eq!(unknown.first().map(|m| m.name), Some("default"));
    }

    #[test]
    fn report_lists_attempts_and_outcome() {
        let attempts = vec![Attempt {
            mode: "default",
            outcome: "falha de segmentação (SIGSEGV)".into(),
            stderr_tail: vec!["Gtk-Message: x".into()],
        }];
        let r = render_report(&attempts, Some("mesa-egl"));
        assert!(r.contains("modo `default`: falha de segmentação"));
        assert!(r.contains("abriu no modo de compatibilidade `mesa-egl`"));
        assert!(r.contains("Gtk-Message: x"));
    }

    #[test]
    fn issue_url_is_encoded_and_bounded() {
        let url = github_issue_url(&"á ".repeat(10_000));
        assert!(url.starts_with("https://github.com/lucacasanova/homedeck/issues/new?"));
        assert!(!url.contains(' '));
        assert!(url.len() < 40_000);
    }

    #[test]
    fn compat_flag_is_stripped() {
        let args: Vec<String> = ["--compat", "x11", "file.mkv"]
            .iter()
            .map(ToString::to_string)
            .collect();
        assert_eq!(strip_compat_flag(&args), vec!["file.mkv".to_owned()]);
    }

    #[test]
    fn exit_descriptions() {
        use std::os::unix::process::ExitStatusExt as _;
        assert_eq!(
            describe_exit(ExitStatus::from_raw(11)),
            "falha de segmentação (SIGSEGV)"
        );
        assert_eq!(
            describe_exit(ExitStatus::from_raw(3 << 8)),
            "saiu com código 3"
        );
    }
}
