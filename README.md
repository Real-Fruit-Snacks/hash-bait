# hash-bait

Drop poisoned shortcuts on SMB shares to coerce NTLM hashes

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/assets/cover-dark.svg">
  <img alt="hash-bait: bait files on a writable share become NetNTLMv2 captures in Responder" src="docs/assets/cover-light.svg" width="100%">
</picture>

```powershell
irm https://raw.githubusercontent.com/Real-Fruit-Snacks/hash-bait/main/install.ps1 | iex
```

```sh
curl -fsSL https://raw.githubusercontent.com/Real-Fruit-Snacks/hash-bait/main/install.sh | sh
```

I got tired of doing this in three tools — one to write the bait files, smbclient to push them, Responder to catch the hashes — and of picking bait out of client shares by hand afterwards. hash-bait is the whole errand as one static binary: it renders `.url`/`.scf`/`.xml` payloads, puts them on the share with its own SMB2/3 stack (no smbclient, no Python), starts Responder, and cleans up after itself. It assumes you have written authorization for the target and a Linux listener on the network. For authorized security testing only.

## What's in it

| Command | Does |
|---|---|
| `hash-bait drop` | Renders bait and puts it on a writable share; starts Responder unless `--no-listen` |
| `hash-bait clean` | Removes previously dropped bait, matched by basename |
| `hash-bait list` | Enumerates shares on a target over IPC$ + srvsvc |
| `hash-bait listen` | Starts Responder on an interface (Linux only) |

`drop` and `clean` take `--dry-run` to print the plan with no side effects, `-u`/`--pass`/`--ask-pass` for authenticated sessions, and `-d` for a subdirectory when the share root is read-only.

## Tested on

- Windows 11 · stable Rust — build host; unit, golden and CLI tests
- ubuntu-24.04 · live Samba share — CI drops, lists and cleans against it on every commit
- *Not* tested: macOS — cross-compiled and unsigned; probably works, no promises

## Known issues

- `listen` needs Responder, which only runs on Linux. On Windows it explains and exits 4.
- Anonymous drops are guest sessions — the Rust SMB stack can't express a true null session, so `guest` + empty password is the portable equivalent. Against servers that accept the session before SSPI completes (Samba `map to guest`, most NAS devices), the `smb` crate currently errors with "SMB session setup succeeded before SSPI authentication completed" — a crate limitation, not fixable from here. Guest access is best-effort; authenticated drops are the tested path (CI proves them against a live Samba share on every commit). Default Windows refuses guest either way.
- The macOS binaries are not notarized; first run needs `xattr -d com.apple.quarantine` on the binary.

## Configuration

- `HASH_BAIT_PASSWORD` — password source between `--pass` and `--ask-pass`
- `NO_COLOR` or `--no-color` — plain output

## License

MIT
