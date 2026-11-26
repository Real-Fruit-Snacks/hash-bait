//! Terminal output in the RFS Cyclorama Dawn palette (BRAND.md §11).
//! Every status line starts with a bracketed ASCII tag — colour is never
//! the only signal, and output stays meaningful under NO_COLOR or a pipe.

use std::sync::atomic::{AtomicBool, Ordering};

static NO_COLOR: AtomicBool = AtomicBool::new(false);
static ANSI16: AtomicBool = AtomicBool::new(false);

/// `disabled` strips all escapes (NO_COLOR, --no-color, TERM=dumb, pipe).
/// `ansi16` picks §11's 16-colour row for terminals without truecolour.
pub fn init(disabled: bool, ansi16: bool) {
    NO_COLOR.store(disabled, Ordering::Relaxed);
    ANSI16.store(ansi16, Ordering::Relaxed);
}

fn on() -> bool {
    !NO_COLOR.load(Ordering::Relaxed)
}

// Cyclorama Dawn tokens (BRAND.md §11), truecolour dark row.
const BEACON: &str = "\x1b[38;2;255;146;84m"; // #FF9254
const MINT: &str = "\x1b[38;2;61;220;151m"; // #3DDC97
const AMBER: &str = "\x1b[38;2;255;194;71m"; // #FFC247
const FAULT: &str = "\x1b[38;2;255;92;108m"; // #FF5C6C Signal Crimson
const STEEL: &str = "\x1b[38;2;116;184;232m"; // #74B8E8
const MUTE: &str = "\x1b[38;2;179;181;192m"; // #B3B5C0
// §11's 16-colour row, for terminals without truecolour.
const BEACON16: &str = "\x1b[91m"; // bright red
const MINT16: &str = "\x1b[32m"; // green
const AMBER16: &str = "\x1b[33m"; // yellow
const FAULT16: &str = "\x1b[31m"; // red
const STEEL16: &str = "\x1b[36m"; // cyan
const MUTE16: &str = "\x1b[90m"; // bright black; §11 defines no 16-colour mute
const RESET: &str = "\x1b[0m";

fn tier(truecolour: &'static str, basic: &'static str) -> &'static str {
    if ANSI16.load(Ordering::Relaxed) {
        basic
    } else {
        truecolour
    }
}

fn paint(code: (&'static str, &'static str), tag: &str, msg: &str) -> String {
    if on() {
        format!("{}{}{RESET} {msg}", tier(code.0, code.1), tag)
    } else {
        format!("{tag} {msg}")
    }
}

/// Something succeeded. Mint is only ever "OK".
pub fn ok(msg: &str) {
    println!("{}", paint((MINT, MINT16), "[ok]", msg));
}

/// Something the user should see before continuing. Amber is caution, not failure.
pub fn warn(msg: &str) {
    eprintln!("{}", paint((AMBER, AMBER16), "[!]", msg));
}

/// Something failed. Signal Crimson, never red — it cannot be mistaken for Beacon.
pub fn fail(msg: &str) {
    eprintln!("{}", paint((FAULT, FAULT16), "[x]", msg));
}

/// A step the tool will take (used by --dry-run). Muted.
pub fn plan(msg: &str) {
    println!("{}", paint((MUTE, MUTE16), "[i]", msg));
}

/// Neutral progress detail. Steel.
pub fn info(msg: &str) {
    println!("{}", paint((STEEL, STEEL16), "[i]", msg));
}

/// The Beacon brand hit: prompt-style lines, used sparingly.
pub fn prompt(msg: &str) {
    println!("{}", paint((BEACON, BEACON16), "$", msg));
}
