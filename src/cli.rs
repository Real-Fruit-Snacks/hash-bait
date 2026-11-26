//! Command-line surface. Binary name is the repo name; subcommands are verbs.

use clap::{Args, Parser, Subcommand};

use crate::payloads::Kind;

pub const DEFAULT_BASENAME: &str = "@bait";
pub const PASSWORD_ENV: &str = "HASH_BAIT_PASSWORD";

#[derive(Parser, Debug)]
#[command(
    name = "hash-bait",
    version,
    about = "Drop poisoned shortcuts on SMB shares to coerce NTLM hashes",
    long_about = "Drop poisoned .url/.scf/.xml shortcuts on writable SMB shares so that \
                  browsing users hand you their NetNTLM authentication. Pair with \
                  Responder for capture. For authorized security testing only.",
    arg_required_else_help = true
)]
pub struct Cli {
    /// Disable colour output (also honours $NO_COLOR)
    #[arg(long, global = true)]
    pub no_color: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Drop bait files onto a share and (optionally) start Responder
    Drop(DropArgs),
    /// Remove previously dropped bait from a share
    Clean(CleanArgs),
    /// Start Responder only (no upload)
    Listen(ListenArgs),
    /// Enumerate shares on a target
    List(ListArgs),
}

#[derive(Args, Debug)]
pub struct AuthArgs {
    /// Username: 'user', 'DOMAIN/user', 'DOMAIN\user', or 'user@DOMAIN.LOCAL'
    #[arg(short = 'u', long, value_name = "USER")]
    pub user: Option<String>,

    /// Password (visible in ps; prefer --ask-pass or $HASH_BAIT_PASSWORD)
    #[arg(long = "pass", value_name = "PASS")]
    pub password: Option<String>,

    /// Prompt for the password interactively (no echo)
    #[arg(long)]
    pub ask_pass: bool,
}

#[derive(Args, Debug)]
pub struct TargetArgs {
    /// Target SMB server (IP or hostname)
    #[arg(short = 't', long, value_name = "HOST")]
    pub target: String,

    /// Target share name
    #[arg(short = 's', long, value_name = "NAME")]
    pub share: String,

    /// Subdirectory inside the share (when the root is read-only)
    #[arg(short = 'd', long, value_name = "PATH")]
    pub dir: Option<String>,
}

#[derive(Args, Debug)]
pub struct PayloadArgs {
    /// Payload type: url, scf, xml, or all (repeatable; default: all)
    #[arg(short = 'p', long, value_name = "TYPE")]
    pub payload: Vec<String>,

    /// Payload basename (the @ prefix sorts it to the top of listings)
    #[arg(long, default_value = DEFAULT_BASENAME, value_name = "BASE")]
    pub name: String,

    /// Print the plan without writing or uploading anything
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Args, Debug)]
pub struct DropArgs {
    #[command(flatten)]
    pub target: TargetArgs,

    /// Attacker IP embedded in payload UNC paths
    #[arg(short = 'c', long, value_name = "IP")]
    pub callback: String,

    /// Network interface for Responder (required unless --no-listen or --dry-run)
    #[arg(short = 'i', long, value_name = "IFACE")]
    pub iface: Option<String>,

    /// Drop only — skip the Responder handoff
    #[arg(long)]
    pub no_listen: bool,

    /// Also write the payload files locally
    #[arg(long)]
    pub save_local: bool,

    #[command(flatten)]
    pub payloads: PayloadArgs,

    #[command(flatten)]
    pub auth: AuthArgs,
}

#[derive(Args, Debug)]
pub struct CleanArgs {
    #[command(flatten)]
    pub target: TargetArgs,

    #[command(flatten)]
    pub payloads: PayloadArgs,

    #[command(flatten)]
    pub auth: AuthArgs,
}

#[derive(Args, Debug)]
pub struct ListenArgs {
    /// Network interface for Responder
    #[arg(short = 'i', long, value_name = "IFACE")]
    pub iface: String,
}

#[derive(Args, Debug)]
pub struct ListArgs {
    /// Target SMB server (IP or hostname)
    #[arg(short = 't', long, value_name = "HOST")]
    pub target: String,

    #[command(flatten)]
    pub auth: AuthArgs,
}

/// Split a user spec into (domain, user), matching the original tool:
/// 'DOMAIN/user', 'DOMAIN\user', 'user@DOMAIN', or bare 'user'.
pub fn parse_user(spec: &str) -> (Option<String>, String) {
    if let Some((domain, user)) = spec.split_once('/') {
        return (Some(domain.to_string()), user.to_string());
    }
    if let Some((domain, user)) = spec.split_once('\\') {
        return (Some(domain.to_string()), user.to_string());
    }
    if let Some((user, domain)) = spec.split_once('@') {
        return (Some(domain.to_string()), user.to_string());
    }
    (None, spec.to_string())
}

/// Password resolution order: --pass > $HASH_BAIT_PASSWORD > --ask-pass prompt.
pub fn resolve_password(auth: &AuthArgs) -> std::io::Result<Option<String>> {
    if let Some(p) = &auth.password {
        return Ok(Some(p.clone()));
    }
    if let Ok(p) = std::env::var(PASSWORD_ENV) {
        if !p.is_empty() {
            return Ok(Some(p));
        }
    }
    if auth.ask_pass {
        return Ok(Some(rpassword::prompt_password("Password: ")?));
    }
    Ok(None)
}

/// Normalize -p/--payload values into an ordered, deduplicated list.
/// Empty / 'all' → all three types in canonical order.
pub fn resolve_payload_types(specs: &[String]) -> Result<Vec<Kind>, String> {
    if specs.is_empty() || specs.iter().any(|s| s == "all") {
        return Ok(Kind::ALL.to_vec());
    }
    let mut kinds = Vec::new();
    for s in specs {
        match Kind::parse(s) {
            Some(k) if !kinds.contains(&k) => kinds.push(k),
            Some(_) => {}
            None => return Err(format!("unknown payload type '{s}' (url, scf, xml, all)")),
        }
    }
    Ok(kinds)
}

/// SMB path fragment for a subdirectory argument: forward slashes become backslashes.
pub fn remote_dir(dir: &str) -> String {
    dir.replace('/', "\\")
}
