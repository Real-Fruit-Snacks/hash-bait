# Security Policy

## Supported Versions

Only the latest release of hash-bait is supported with security updates.

| Version  | Supported |
| -------- | --------- |
| latest   | yes       |
| < latest | no        |

## Reporting a Vulnerability

**Do not open public issues for security vulnerabilities.**

If you discover a security vulnerability in hash-bait, please report it responsibly:

1. **Preferred:** use [GitHub Security Advisories](https://github.com/Real-Fruit-Snacks/hash-bait/security/advisories/new) to create a private report.
2. **Alternative:** email the maintainer directly with details of the vulnerability.

You can expect an acknowledgement within a few days and a fix or a stated reason there is none as soon as practical.

## Scope

hash-bait is an offensive-security tool for authorized testing. Payload files that coerce NTLM authentication are the product, not a vulnerability. Issues in scope: credential handling, memory safety, command injection through arguments, and anything that puts the *operator* — rather than the target — at risk.
