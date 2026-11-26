//! Terminal output in the RFS Nightshift palette.
//! Every status line carries a word — colour is never the only signal.

use std::sync::atomic::{AtomicBool, Ordering};

static NO_COLOR: AtomicBool = AtomicBool::new(false);

pub fn init(disabled: bool) {
    NO_COLOR.store(disabled, Ordering::Relaxed);
}

fn on() -> bool {
    !NO_COLOR.load(Ordering::Relaxed)
}

// Nightshift tokens (BRAND.md §1), as ANSI truecolour escapes.
const EMBER: &str = "\x1b[38;2;255;106;61m";
const MINT: &str = "\x1b[38;2;61;220;151m";
const AMBER: &str = "\x1b[38;2;255;194;71m";
const FAULT: &str = "\x1b[38;2;240;98;146m";
const MUTE: &str = "\x1b[38;2;138;143;153m";
const RESET: &str = "\x1b[0m";

fn paint(code: &'static str, word: &str, msg: &str) -> String {
    if on() {
        format!("{code}{word}{RESET} {msg}")
    } else {
        format!("{word} {msg}")
    }
}

/// Something succeeded. Mint is only ever "OK".
pub fn ok(msg: &str) {
    println!("{}", paint(MINT, "ok", msg));
}

/// Something the user should see before continuing. Amber is caution, not failure.
pub fn warn(msg: &str) {
    eprintln!("{}", paint(AMBER, "warn", msg));
}

/// Something failed. Raspberry, never red — it cannot be mistaken for Ember.
pub fn fail(msg: &str) {
    eprintln!("{}", paint(FAULT, "fail", msg));
}

/// A step the tool will take (used by --dry-run). Muted, no status colour.
pub fn plan(msg: &str) {
    println!("{}", paint(MUTE, "plan", msg));
}

/// Neutral progress detail. Muted.
pub fn info(msg: &str) {
    println!("{}", paint(MUTE, "*", msg));
}

/// The Ember brand hit: prompt-style lines, used sparingly.
pub fn ember(msg: &str) {
    println!("{}", paint(EMBER, ">", msg));
}
