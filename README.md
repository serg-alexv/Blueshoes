# Blueshoes

Blueshoes is an adaptive networking runtime doctrine and reference implementation designed to make constrained network behavior observable, transactional, and recoverable.

It operates primarily on extremely constrained edge hardware (home routers running OpenWrt) to provide resilient routing without risking total loss of internet connectivity.

## Core Philosophy: Rollback is Sacred
Configuring firewall and routing rules programmatically is dangerous. If a rule goes wrong, the router drops offline permanently and the user is stranded.

Blueshoes solves this by making network mutation **transactional**. The system snapshots the active routing state, applies a new strategy, runs an instant validation check, and if the check fails, the system rolls back to the snapshot within 5 seconds.

## Phase 1 Scope
Phase 1 targets the **GL.iNet GL-MT3000** (OpenWrt) with a deterministic edge agent (`bs-edge-agent`). 
- LLM diagnostics remain external/read-only on a separate workbench (`bs-workbench`).
- MITM is strictly forbidden.
- ECH is observed and preserved, not forced.
- Blueshoes does not ship with bundled commercial VPN endpoints or “one-click paid tunnel” defaults.

Read more in [Phase 1 Scope](docs/rfcs/0012-phase1-scope.md).

The project architecture and doctrine are maintained as an RFC corpus. See the [docs/rfcs](docs/rfcs/) directory for the complete doctrine surface.

- [Runtime Doctrine](docs/rfcs/0001-runtime-doctrine.md)
- [Rollback Model](docs/rfcs/0002-rollback-model.md)
- [Security Policy](SECURITY.md)

## Status
**Development Stage.** No production code has been released yet. We are actively scaffolding the `bs-edge-agent` native binary.
