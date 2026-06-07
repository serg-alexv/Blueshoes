# MITM Ban Policy

**Blueshoes absolutely forbids TLS interception, SSL stripping, or any form of transparent Man-In-The-Middle (MITM) architecture for production traffic.**

## Why?
1. **Security Degradation**: Installing custom root CAs on client devices fundamentally weakens their security posture.
2. **Protocol Breaking**: Transparent MITM breaks advanced cryptographic features like Encrypted Client Hello (ECH) and certificate pinning, which are critical for censorship resistance.
3. **Privacy Violation**: A router should never inspect the plaintext payloads of user traffic.

## Permitted Exceptions (Lab Environment Only)
See `lab-tools.md` for explicit, client-cooperative diagnostic exceptions used purely for research and development.
