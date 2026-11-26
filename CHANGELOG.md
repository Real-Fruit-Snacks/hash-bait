# Changelog

Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Versions: [SemVer](https://semver.org/).

## [Unreleased]

## [1.0.0] - 2026-11-25

hash-bait is the Rust successor to Lure — same job, one static binary, no smbclient or Python required.

### Added

- `drop` — renders `.url`/`.scf`/`.xml` bait and uploads it over a native SMB2/3 stack (signing and encryption supported), then starts Responder unless `--no-listen`
- `clean` — removes previously dropped bait, matched by basename; missing files are reported, not fatal
- `list` — share enumeration over IPC$ + srvsvc
- `listen` — Responder handoff on Linux; explains and exits on other platforms
- `--dry-run` on `drop` and `clean` prints the plan with no side effects
- `--save-local` on `drop` also writes the payload files locally
- Payload bytes pinned to the original tool's output by golden tests; every commit is dropped, listed and cleaned against a live Samba share in CI
