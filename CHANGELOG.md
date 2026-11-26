# Changelog

Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Versions: [SemVer](https://semver.org/).

## [1.0.1] - 2026-11-25

### Added

- Project page at <https://real-fruit-snacks.github.io/hash-bait/> (docs/, GitHub Pages)
- Generated README cover and social preview (`docs/assets/`), rendered by `rfs-brand/tools/make-cover.py`
- `AGENTS.md` (RFS Cyclorama Dawn guardrails) and `SECURITY.md`
- Colour handling now also honours `TERM=dumb`, and terminals without truecolour (`TERM=linux`, or a `COLORTERM` other than `truecolor`/`24bit`) get the CLI standard's 16-colour row in place of the truecolour escapes

### Changed

- Terminal output follows the Cyclorama Dawn CLI standard: bracketed `[ok]` / `[!]` / `[x]` / `[i]` tags and the Beacon prompt, with color also stripped when stdout is not a terminal (`NO_COLOR` and `--no-color` unchanged)
- `--version` prints `hash-bait v1.0.1`, the standard's `<tool> vX.Y.Z`
- CI proves `drop`/`list`/`clean` against a live Windows share on windows-2022; the Samba job asserts authenticated `drop`/`clean`, with `list` and the guest drop as informational probes
- Release packaging no longer ships a bare `hash-bait.exe` next to the Windows zip

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
