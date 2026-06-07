# Blueshoes

Blueshoes is a rollback-safe adaptive networking runtime doctrine and reference implementation for constrained edge devices.

It operates primarily on OpenWrt-based routers to provide resilient routing, bounded recovery, and deterministic rollback without risking total loss of internet connectivity.

## Active Milestone: M4.1 Safety Containment

The B0 architecture is governed by strict deterministic planning. **The M4.1 Planner is strictly a planning engine. It does NOT mutate state by default.** 

All `bs-edge-agent` execution capabilities are locked behind an explicit `--unsafe-execute` CLI flag and isolated by a `dangerous_execution` compile-time feature. 

**Default Behavior**:
```bash
bs-edge-agent canary
```
*Outputs a dry-run log of JSON commands. Execution is aborted safely.*

**Execution Override**:
```bash
bs-edge-agent --unsafe-execute canary
```
*Triggers the actual physical execution of the planned mutation (Only allowed if compiled with `--features dangerous_execution`).*

## Suggested GitHub “About” (Copy/Paste)

- **Description**: Transactional networking runtime for OpenWrt routers: observe failures, apply bounded profiles, validate fast, and roll back safely (“Rollback is Sacred”). No MITM. No opaque automation. No bundled paid VPN defaults.
- **Topics**: openwrt, router, networking, rollback, reliability, rust, dns, ech, edge-computing, observability, censorship-resilience

## Core Philosophy: Rollback is Sacred

Programmatic routing mutation is dangerous. A broken firewall or routing rule can permanently disconnect the user from the network.

Blueshoes treats every routing mutation as a bounded transaction:

1. Observe the current state.
2. Apply a constrained profile.
3. Validate connectivity.
4. Roll back automatically on failure.

The runtime must fail safely, deterministically, and recoverably.

## Scope

Phase 1 targets the GL.iNet GL-MT3000 (OpenWrt) with a deterministic edge agent written in Rust.

## Explicit Constraints

- No MITM/TLS interception.
- No autonomous shell mutation by LLMs.
- ECH is observed and preserved, not forced.
- Blueshoes does not ship with bundled commercial VPN endpoints or “one-click paid tunnel” defaults.
- No opaque orchestration layers in the runtime path.
- Human override remains mandatory for high-risk operations.

Read more in [Phase 1 Scope](docs/phase1-scope.md).

- [Core Doctrine](docs/doctrine.md)
- [System Architecture](docs/architecture.md)
- [MITM Ban](docs/mitm-ban.md)
- [ECH Position](docs/ech-position.md)
- [Profiles](docs/profiles.md)
- [Transaction Model](docs/transaction-model.md)

- [Security Policy](SECURITY.md)

## Status

Current status: B0 Runtime Beta Pack.

The runtime currently supports:
- Read-only telemetry probes
- Structured transaction journaling
- Cross-compilation for OpenWrt targets
- Deterministic audit validation

The runtime does NOT yet mutate routing state.
