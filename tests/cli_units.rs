//! CLI unit tests: user-spec parsing, payload resolution, and argument
//! validation — the surface the original tool's argparse covered.

use clap::{error::ErrorKind, CommandFactory, Parser};
use hash_bait::cli::{parse_user, remote_dir, resolve_payload_types, Cli};
use hash_bait::payloads::Kind;

#[test]
fn parses_plain_user() {
    assert_eq!(parse_user("jdoe"), (None, "jdoe".to_string()));
}

#[test]
fn parses_domain_slash() {
    assert_eq!(
        parse_user("CORP/jdoe"),
        (Some("CORP".to_string()), "jdoe".to_string())
    );
}

#[test]
fn parses_domain_backslash() {
    assert_eq!(
        parse_user("CORP\\jdoe"),
        (Some("CORP".to_string()), "jdoe".to_string())
    );
}

#[test]
fn parses_upn() {
    assert_eq!(
        parse_user("jdoe@corp.local"),
        (Some("corp.local".to_string()), "jdoe".to_string())
    );
}

#[test]
fn payload_default_is_all() {
    assert_eq!(resolve_payload_types(&[]).unwrap(), Kind::ALL.to_vec());
}

#[test]
fn payload_all_keyword_is_all() {
    assert_eq!(
        resolve_payload_types(&["all".to_string()]).unwrap(),
        Kind::ALL.to_vec()
    );
}

#[test]
fn payload_dedupes_repeats() {
    assert_eq!(
        resolve_payload_types(&["scf".to_string(), "scf".to_string()]).unwrap(),
        vec![Kind::Scf]
    );
}

#[test]
fn payload_rejects_unknown() {
    assert!(resolve_payload_types(&["hta".to_string()]).is_err());
}

#[test]
fn remote_dir_converts_slashes() {
    assert_eq!(remote_dir("HR/public/docs"), "HR\\public\\docs");
}

#[test]
fn clap_requires_subcommand() {
    let err = Cli::try_parse_from(["hash-bait"]).unwrap_err();
    assert_eq!(
        err.kind(),
        ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
    );
}

#[test]
fn clap_requires_callback_on_drop() {
    let err =
        Cli::try_parse_from(["hash-bait", "drop", "-t", "10.10.10.5", "-s", "public"]).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
fn clap_debug_asserts() {
    Cli::command().debug_assert();
}
