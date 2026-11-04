//! Startup supervisor (ADR 0010).
//!
//! The binary first runs as a light supervisor (no WebKit, no GPU). It starts
//! the real UI as a child process and waits for a one-time token the child
//! prints on its own stdout once the page has loaded. If the child dies before
//! that — typically a graphics driver crash — it writes a local report and
//! retries with the next compatibility mode. The mode that works is remembered.
//!
//! No marker files: the ready signal travels over the child's stdout pipe,
//! which only the supervisor can read.

use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Read, Write as _};
use std::path::Path;
use std::process::{Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::diagnostics::{
    config_dir, data_dir, environment_report, log_tail, redact, write_private,
};

/// Argument that marks a process as the UI child; followed by the ready token.
/// An argument (not an env var) so programs the UI launches do not inherit it.
pub const CHILD_ARG: &str = "--homedeck-child";
/// Env var with the compatibility mode in use (for logs and the UI).
pub const MODE_ENV: &str = "HOMEDECK_COMPAT";

/// How long a child may take to load the UI. A child that is still alive but
/// never loads (e.g. an empty grey window after a WebKit/EGL failure) is a
/// startup failure: it is stopped and the next mode is tried.
const READY_TIMEOUT: Duration = Duration::from_secs(20);
/// Grace period after forwarding SIGTERM before killing the child.
const STOP_GRACE: Duration = Duration::from_secs(5);
/// Longest line kept from the child's output.
const MAX_LINE: usize = 1024;
/// Lines of child stderr kept for the report.
const MAX_LINES: usize = 60;

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

fn load_saved_mode() -> Option<String> {
    std::fs::read_to_string(config_dir()?.join("compat-mode"))
        .ok()
        .map(|s| s.trim().to_owned())
        .filter(|s| !s.is_empty())
}

fn save_mode(name: &str) {
    if let Some(dir) = config_dir() {
        let _ = write_private(&dir.join("compat-mode"), name.as_bytes());
    }
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

/// Unpredictable one-time token (from the kernel RNG).
fn new_token() -> String {
    let mut buf = [0u8; 16];
    let ok = std::fs::File::open("/dev/urandom")
        .and_then(|mut f| f.read_exact(&mut buf))
        .is_ok();
    if !ok {
        // Extremely unlikely; still unique per run.
        buf[..8].copy_from_slice(&now_secs().to_le_bytes());
        buf[8..12].copy_from_slice(&std::process::id().to_le_bytes());
    }
    let hex: String = buf.iter().map(|b| format!("{b:02x}")).collect();
    format!("HOMEDECK-READY-{hex}")
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

/// Builds the plain-text crash report (already redacted).
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

/// Saves the report (0600) and marks it for the UI to offer later (#128).
/// `pending` holds only the file name; the UI resolves it inside `crash/`.
fn write_report(text: &str) -> Option<std::path::PathBuf> {
    let dir = data_dir()?.join("crash");
    let name = format!("{}.txt", now_secs());
    let path = dir.join(&name);
    write_private(&path, text.as_bytes()).ok()?;
    let _ = write_private(&dir.join("pending"), name.as_bytes());
    Some(path)
}

fn truncate_line(mut line: String) -> String {
    if line.len() > MAX_LINE {
        let mut cut = MAX_LINE;
        while !line.is_char_boundary(cut) {
            cut -= 1;
        }
        line.truncate(cut);
        line.push('…');
    }
    line
}

/// Reads lines with a hard cap per line, so a runaway line cannot grow memory.
fn bounded_lines<R: Read>(reader: R, mut on_line: impl FnMut(String)) {
    let mut reader = BufReader::new(reader);
    let mut buf = Vec::new();
    loop {
        buf.clear();
        match reader.by_ref().take(64 * 1024).read_until(b'\n', &mut buf) {
            Ok(0) | Err(_) => break,
            Ok(_) => {
                let text = String::from_utf8_lossy(&buf)
                    .trim_end_matches(['\n', '\r'])
                    .to_owned();
                on_line(truncate_line(text));
            }
        }
    }
}

enum ChildResult {
    /// UI became ready; carries the final exit status.
    Ran(ExitStatus),
    /// Died before the UI was ready.
    Failed(Attempt),
}

/// Set when the supervisor receives SIGTERM/SIGINT/SIGHUP; the child is then
/// asked to stop too, so no UI window is ever left orphaned.
fn termination_flag() -> Arc<AtomicBool> {
    use signal_hook::consts::{SIGHUP, SIGINT, SIGTERM};
    let flag = Arc::new(AtomicBool::new(false));
    for sig in [SIGTERM, SIGINT, SIGHUP] {
        let _ = signal_hook::flag::register(sig, Arc::clone(&flag));
    }
    flag
}

fn stop_child(child: &mut std::process::Child) -> std::io::Result<ExitStatus> {
    use nix::sys::signal::{Signal, kill};
    use nix::unistd::Pid;
    if let Ok(pid) = i32::try_from(child.id()) {
        let _ = kill(Pid::from_raw(pid), Signal::SIGTERM);
    }
    let deadline = Instant::now() + STOP_GRACE;
    while Instant::now() < deadline {
        if let Some(status) = child.try_wait()? {
            return Ok(status);
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    let _ = child.kill();
    child.wait()
}

fn run_child(
    mode: &Mode,
    args: &[String],
    stop: &AtomicBool,
    on_ready: &mut dyn FnMut(),
) -> std::io::Result<ChildResult> {
    let exe = std::env::current_exe()?;
    let token = new_token();

    let mut cmd = Command::new(exe);
    cmd.arg(CHILD_ARG)
        .arg(&token)
        .args(args)
        .env(MODE_ENV, mode.name)
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (k, v) in &mode.env {
        cmd.env(k, v);
    }
    let mut child = cmd.spawn()?;

    let ready = Arc::new(AtomicBool::new(false));
    // stdout: watch for the token, mirror everything else.
    let out_reader = child.stdout.take().map(|out| {
        let ready = Arc::clone(&ready);
        let token = token.clone();
        std::thread::spawn(move || {
            bounded_lines(out, |line| {
                if line.trim() == token {
                    ready.store(true, Ordering::SeqCst);
                } else {
                    let _ = writeln!(std::io::stdout(), "{line}");
                }
            });
        })
    });
    // stderr: mirror and keep the last lines for the report.
    let tail: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(VecDeque::new()));
    let err_reader = child.stderr.take().map(|err| {
        let tail = Arc::clone(&tail);
        std::thread::spawn(move || {
            bounded_lines(err, |line| {
                let _ = writeln!(std::io::stderr(), "{line}");
                if let Ok(mut t) = tail.lock() {
                    if t.len() == MAX_LINES {
                        t.pop_front();
                    }
                    t.push_back(line);
                }
            });
        })
    });

    let started = Instant::now();
    let mut became_ready = false;
    let mut hung = false;
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if stop.load(Ordering::Relaxed) {
            break stop_child(&mut child)?;
        }
        if !became_ready && ready.load(Ordering::SeqCst) {
            became_ready = true;
            log_line(&format!("modo `{}` abriu a interface", mode.name));
            // Persist the outcome now: the supervisor may be killed later.
            on_ready();
        }
        if !became_ready && started.elapsed() > READY_TIMEOUT {
            hung = true;
            break stop_child(&mut child)?;
        }
        std::thread::sleep(Duration::from_millis(100));
    };
    for r in [out_reader, err_reader].into_iter().flatten() {
        let _ = r.join();
    }

    if became_ready {
        return Ok(ChildResult::Ran(status));
    }
    let stderr_tail = tail
        .lock()
        .map(|t| t.iter().cloned().collect())
        .unwrap_or_default();
    let outcome = if hung {
        format!(
            "a interface não carregou em {} s (janela vazia)",
            READY_TIMEOUT.as_secs()
        )
    } else {
        describe_exit(status)
    };
    Ok(ChildResult::Failed(Attempt {
        mode: mode.name,
        outcome,
        stderr_tail,
    }))
}

fn log_line(msg: &str) {
    let _ = writeln!(std::io::stderr(), "[homedeck] {msg}");
}

/// Shown only when no mode works: a native GTK dialog (no WebKit, no GPU).
/// Nothing leaves the machine unless the user attaches the file themselves.
fn show_fatal_dialog(report: Option<&Path>) {
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
             Ele não sai do seu computador. Para pedir ajuda, abra uma issue e anexe o arquivo."
        ))
        .set_buttons(MessageButtons::YesNoCancelCustom(
            "Abrir pasta".into(),
            "Reportar no GitHub".into(),
            "Fechar".into(),
        ))
        .show();
    let open_folder = || {
        if let Some(dir) = report.and_then(Path::parent) {
            let _ = Command::new("xdg-open").arg(dir).spawn();
        }
    };
    match result {
        MessageDialogResult::Custom(c) if c == "Abrir pasta" => open_folder(),
        MessageDialogResult::Custom(c) if c == "Reportar no GitHub" => {
            open_folder();
            let _ = Command::new("xdg-open").arg(github_issue_url()).spawn();
        }
        _ => {}
    }
}

/// New-issue page with instructions only: no report data travels in the URL.
#[must_use]
pub fn github_issue_url() -> String {
    let body = "O homedeck não abriu neste computador.\n\n\
                **Anexe aqui o arquivo de relatório** (a pasta foi aberta: arraste o `.txt` mais recente para esta caixa).\n\
                Revise o conteúdo antes de enviar.\n";
    format!(
        "https://github.com/lucacasanova/homedeck/issues/new?labels=type%3Abug%2Cplatform%3Alinux&title={}&body={}",
        percent_encode("O homedeck não abre (falha de inicialização)"),
        percent_encode(body)
    )
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

/// If this process is the UI child, returns the ready token it must print.
#[must_use]
pub fn child_token() -> Option<String> {
    let mut args = std::env::args().skip(1);
    (args.next().as_deref() == Some(CHILD_ARG))
        .then(|| args.next())
        .flatten()
}

/// Entry point for `main`. Returns `Some(exit_code)` when the supervisor (or a
/// CLI flag) handled the run, or `None` when this process should start the UI.
#[must_use]
pub fn supervise() -> Option<i32> {
    if child_token().is_some() {
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
                let path = write_report(&render_report(&attempts, Some(mode.name)));
                log_line(&format!(
                    "recuperado no modo `{}`; relatório em {}",
                    mode.name,
                    // Local terminal only: show the real path so the user can open it.
                    path.map(|p| p.display().to_string()).unwrap_or_default()
                ));
            }
            if forced.is_none() && saved.as_deref() != Some(mode.name) {
                save_mode(mode.name);
            }
        };
        match run_child(mode, &child_args, &stop, &mut on_ready) {
            Ok(ChildResult::Ran(status)) => return Some(status.code().unwrap_or(1)),
            Ok(ChildResult::Failed(_)) if stop.load(Ordering::Relaxed) => return Some(1),
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

    let path = write_report(&render_report(&attempts, None));
    show_fatal_dialog(path.as_deref());
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
    fn issue_url_carries_no_report_data() {
        let url = github_issue_url();
        assert!(url.starts_with("https://github.com/lucacasanova/homedeck/issues/new?"));
        assert!(!url.contains(' '));
        assert!(url.len() < 2000);
    }

    #[test]
    fn tokens_are_unique_and_unguessable_format() {
        let (a, b) = (new_token(), new_token());
        assert_ne!(a, b);
        assert!(a.starts_with("HOMEDECK-READY-") && a.len() == "HOMEDECK-READY-".len() + 32);
    }

    #[test]
    fn long_lines_are_truncated_on_char_boundary() {
        let line = "á".repeat(MAX_LINE);
        let out = truncate_line(line);
        assert!(out.len() <= MAX_LINE + '…'.len_utf8());
        assert!(out.ends_with('…'));
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
