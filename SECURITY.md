# Security Policy

Blueshoes operates at the lowest level of the network stack, touching routing tables, DNS resolution, and packet telemetry. We take security and privacy extremely seriously.

## Supported Versions
Only the `main` branch is currently receiving active security updates as this is a Phase 1 research project.

## Reporting a Vulnerability
If you discover a vulnerability, do NOT open a public issue. Please email `security@timelabs.ru` (or the equivalent maintainer address) directly. We will acknowledge your report within 48 hours.

## Core Security Commitments
1. **No MITM**: Blueshoes will never install synthetic root certificates or decrypt TLS payloads.
2. **LLM Isolation**: Diagnostic LLMs are strictly read-only and operate entirely outside the router's execution environment. They cannot issue state-mutating shell commands.
3. **Data Privacy**: Telemetry exported for diagnostic routing analysis must be strictly metadata (SNI, IP, ports, connection status) and fully anonymized. Raw payloads are never exported.
