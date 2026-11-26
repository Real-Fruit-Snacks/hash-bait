//! Terminal output in the RFS Cyclorama Dawn palette (BRAND.md §11).
//! Every status line starts with a bracketed ASCII tag — colour is never
//! the only signal, and output stays meaningful under NO_COLOR or a pipe.

use std::sync::atomic::{AtomicBool, Ordering};

static NO_COLOR: AtomicBool = AtomicBool::new(false);

pub fn init(disabled: bool) {
    NO_COLOR.store(disabled, Ordering::Relaxed);
}

fn on() -> bool {
    !NO_COLOR.load(Ordering::Relaxed)
}

// Cyclorama Dawn tokens (BRAND.md §11), as ANSI truecolour escapes.
const BEACON: &str = "\x1b[38;2;255;146;84m"; // #FF9254
const MINT: &str = "\x1b[38;2;61;220;151m"; // #3DDC97
const AMBER: &str = "\x1b[38;2;255;194;71m"; // #FFC247
const FAULT: &str = "\x1b[38;2;255;92;108m"; // #FF5C6C Signal Crimson
const STEEL: &str = "\x1b[38;2;116;184;232m"; // #74B8E8
const MUTE: &str = "\x1b[38;2;179;181;192m"; // #B3B5C0
const RESET: &str = "\x1b[0m";

fn paint(code: &'static str, tag: &str, msg: &str) -> String {
    if on() {
        format!("{code}{tag}{RESET} {msg}")
    } else {
        format!("{tag} {msg}")
    }
}

/// Something succeeded. Mint is only ever "OK".
pub fn ok(msg: &str) {
    println!("{}", paint(MINT, "[ok]", msg));
}

/// Something the user should see before continuing. Amber is caution, not failure.
pub fn warn(msg: &str) {
    eprintln!("{}", paint(AMBER, "[!]", msg));
}

/// Something failed. Signal Crimson, never red — it cannot be mistaken for Beacon.
pub fn fail(msg: &str) {
    eprintln!("{}", paint(FAULT, "[x]", msg));
}

/// A step the tool will take (used by --dry-run). Muted.
pub fn plan(msg: &str) {
    println!("{}", paint(MUTE, "[i]", msg));
}

/// Neutral progress detail. Steel.
pub fn info(msg: &str) {
    println!("{}", paint(STEEL, "[i]", msg));
}

/// The Beacon brand hit: prompt-style lines, used sparingly.
pub fn prompt(msg: &str) {
    println!("{}", paint(BEACON, "$", msg));
}
