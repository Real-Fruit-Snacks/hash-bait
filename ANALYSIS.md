# Analysis — Lure → hash-bait

Why this tool exists, what the rewrite changed, and what was verified. Written
as part of the RFS revival review (2026-11-25); the original repo was retired
the day hash-bait shipped.

## The original

**Lure** (Python, ~545 LOC, single module) dropped poisoned `.url`/`.scf`/`.xml`
files onto writable SMB shares so browsing users would hand their NetNTLM
authentication to Responder. Four subcommands: `drop`, `clean`, `listen`,
`list`.

### Feature inventory

| Feature | Mechanism | Kept? |
| --- | --- | --- |
| Payload generation (`.url`, `.scf`, `.xml`) | string templates | Yes — byte-exact |
| Upload / delete on shares | system `smbclient` subprocess | Yes — native SMB2/3 stack instead |
| Share enumeration | `smbclient -L` | Yes — IPC$ + srvsvc, native |
| Responder handoff | `sudo responder -I <iface> -wv` | Yes — unchanged (Linux only) |
| Password sources | flag > env > prompt | Yes — env renamed (below) |
| Dry run | printed the smbclient command | Yes — now prints the plan |
| Catppuccin Mocha palette, ASCII banner | hardcoded ANSI | No — Nightshift palette, no banner |

### Weaknesses found in review

1. **smbclient was the real engine.** The "small tool" only stayed small by
   outsourcing SMB to the system — and smbclient doesn't exist on Windows, so
   the tool was Linux-only in practice.
2. **No tests.** Payload bytes, user parsing, dry-run output: nothing pinned.
3. Anonymous vs authenticated upload both worked, but only through smbclient's
   `-N` flag semantics, never exercised automatically.

## Niche check (2026)

Does the hole still exist? Yes:

- **ntlm_theft** generates ~20 bait file types but neither uploads nor cleans.
- **NetExec** has a `drop-scf` module — one file type, no cleanup command.
- Nobody else does *generate → upload → start listener → clean up* as one
  command. The `clean` half alone justifies the tool: bait left on a client
  share is what gets testers caught.

Verdict: **rewrite in Rust.** The whole tool is templates + SMB put/del + a
process handoff; the `smb` crate (pure-Rust SMB2/3, actively maintained,
MIT) removes the smbclient dependency and makes the Windows build real.
rust-anything-style verification applied: the original's behavior was
recorded (payload bytes + dry-run transcripts) before a line of Rust was
written, and the port is tested against those recordings.

## Behavior parity

**Byte-exact** (pinned by golden tests in `tests/golden_payloads.rs`):

- `.url`, `.scf`, `.xml` payload contents for any callback IP / basename.
- User-spec parsing: `user`, `DOMAIN/user`, `DOMAIN\user`, `user@DOMAIN`.
- Password resolution order: `--pass` > env > `--ask-pass` prompt.
- Exit code 2 for usage errors, 4 for missing Responder.

**Intentional divergences:**

| Original | hash-bait | Why |
| --- | --- | --- |
| Payload line endings followed the OS (CRLF on Windows, LF on Linux) | always LF | matches the original's Linux (Kali) behavior; all three file formats accept LF |
| Always wrote local files, then uploaded | uploads directly; `--save-local` for local copies | no more smbclient staging files left in your cwd |
| Default basename `@lure` | `@bait` | old name retired; `@` prefix kept for listing sort |
| Env var `LURE_PASSWORD` | `HASH_BAIT_PASSWORD` | renamed with the tool |
| Dry run printed the smbclient command | dry run prints the plan (render / put / listen) | there is no smbclient command any more |
| `list` via `smbclient -L` | native IPC$ + srvsvc (`NetShareEnumAll`) | the smb crate grew RPC support; the feature survived the cut list |
| ASCII-art banner, Catppuccin Mocha | no banner, Nightshift palette with `ok`/`warn`/`fail` words | RFS brand: real tools, no filler; status is never color-only |

## Verification

- 18 tests green on Windows 11 (stable Rust): payload goldens, CLI parsing,
  clap validation, canonical payload ordering.
- `cargo clippy --all-targets -- -D warnings` clean; `cargo fmt` enforced.
- CI (every commit): the above plus a **live Samba integration run** —
  anonymous drop, share list, and clean against a real smb server, asserting
  the files appear and disappear.
- Manual smoke: dry-run transcripts and exit codes match the original's
  recorded behavior.

## Risks / honest gaps

- **The SMB stack is young.** `smb` 0.12 is pure Rust and maintained, but has
  nowhere near impacket's mileage. Oddball servers (old Samba, printers, NAS
  firmware) may misbehave in ways smbclient wouldn't.
- **Anonymous drops are guest sessions.** The NTLM stack cannot express a
  true null session (empty identity is rejected), so `guest` + empty
  password is the portable equivalent — caught by CI against Samba's
  `map to guest` on day one. Default Windows refuses it: loudly, not
  silently.
- macOS binaries are cross-compiled and unsigned — untested.

## Roadmap candidates (not in v1.0.0)

- More payload types (`.lnk`, `.library-ms`, Office templates) behind the same
  `-p` flag.
- Built-in NetNTLM capture listener, removing the Responder dependency.
- Windows exe version metadata via `embed-resource`.
