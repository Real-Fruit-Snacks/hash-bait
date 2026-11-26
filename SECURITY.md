# Security Policy

## Scope

Real-Fruit-Snacks tools run locally on your machine with no hosted backend, no user accounts, and no telemetry. Security reports apply to local privilege handling, credential or certificate storage, command execution, file parsing, and release integrity.

Note the dual nature of this one: hash-bait is an offensive tool, written for authorized security testing with explicit written permission. Bugs in how it handles credentials, remote shares or the Responder subprocess are in scope; the fact that it coerces NTLM authentication is the feature, not a vulnerability.

## Reporting a Vulnerability

Please do **not** open a public GitHub issue for an undisclosed vulnerability.

1. Use **GitHub Private Vulnerability Reporting** under this repository's **Security** tab (`Security -> Advisories -> Report a vulnerability`).
2. Include the affected version (`hash-bait --version`), OS/platform, reproduction steps, and expected vs. actual behavior.

You will receive an acknowledgement within 7 days and a plain-language fix or mitigation plan once reproduced.
