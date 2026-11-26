//! Responder handoff. Responder itself stays the capture tool; hash-bait
//! does not reimplement it. Unix only — Responder needs raw sockets and is
//! a Kali/Debian citizen.

#[cfg(unix)]
use std::process::Command;
use std::process::ExitStatus;

use crate::out;

/// True if `responder` is on PATH.
#[cfg(unix)]
fn responder_available() -> bool {
    std::env::var_os("PATH")
        .map(|paths| {
            std::env::split_paths(&paths).any(|dir| {
                let candidate = dir.join("responder");
                candidate.is_file()
            })
        })
        .unwrap_or(false)
}

/// Start Responder on the given interface, inheriting the terminal.
/// Exit 4 when Responder is not installed (matches the original tool).
#[cfg(unix)]
pub fn run(iface: &str) -> Result<ExitStatus, String> {
    if !responder_available() {
        out::fail("missing required tool on $PATH: responder");
        out::warn("install on Kali / Debian: sudo apt install responder");
        std::process::exit(4);
    }
    out::info(&format!("starting Responder on {iface} — Ctrl-C to stop"));
    Command::new("sudo")
        .args(["responder", "-I", iface, "-wv"])
        .status()
        .map_err(|e| format!("failed to launch responder: {e}"))
}

#[cfg(not(unix))]
pub fn run(_iface: &str) -> Result<ExitStatus, String> {
    out::fail("the listen command needs Responder, which only runs on Linux");
    out::warn("run the drop from this machine, run 'hash-bait listen' on your Kali box");
    std::process::exit(4);
}
