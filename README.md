# Blueshoes

Blueshoes is an adaptive networking runtime doctrine and reference implementation: network state becomes observable, transactional, recoverable, and rollback-safe.

## What is Blueshoes?
Blueshoes is an architecture and software suite for open-source routers (primarily OpenWrt) designed to combat deep packet inspection (DPI) and internet fragmentation. Instead of relying on brittle bash scripts or single points of failure (like a standard VPN), Blueshoes treats routing as a transactional state machine that dynamically adapts to network pathologies.

## What problem does it solve?
In highly restricted networking environments, standard censorship evasion tools (WireGuard, OpenVPN, standard proxies) are easily identified and blocked by ISPs via TCP Resets or DNS poisoning. When a VPN tunnel dies, the user is often left offline with no automated recovery. Blueshoes guarantees that a router will test new obfuscation profiles, and if they fail, immediately roll back to a known-good state, ensuring maximum uptime and accessibility.

## What it is not
- It is **not** a traditional VPN provider.
- It is **not** a Man-in-the-Middle (MITM) proxy. We do not decrypt your TLS traffic.
- It is **not** a "magic" AI box. While we use LLMs for off-router diagnostics, the router itself executes deterministic, predictable logic.

## Phase 1 Scope
Phase 1 targets the **GL.iNet GL-MT3000** (OpenWrt) with a deterministic edge agent (`bs-edge-agent`). 
- LLM diagnostics remain external/read-only on a separate workbench (`bs-workbench`).
- MITM is strictly forbidden.
- ECH is observed and preserved, not forced.
Read more in [Phase 1 Scope](docs/phase1-scope.md).

## Architecture
The system is bifurcated:
1. **The Edge Agent (`bs-edge-agent`)**: A highly constrained, memory-safe daemon running on the router that handles connection observation, atomic profile switching, and rollbacks.
2. **The Workbench (`bs-workbench`)**: An external environment (VM or laptop) where heavy diagnostics, PCAP analysis, and LLM telemetry parsing occur.

## Safety Doctrine
We operate under strict engineering constraints to protect the router and the user:
1. **Rollback is sacred**: Any profile change that breaks internet access is reverted in < 5 seconds.
2. **Non-destructive**: Removing Blueshoes restores the router to its vanilla state.
3. **No AI in the control loop**: LLMs suggest profiles; they cannot write `iptables` rules directly.
Read the full [Doctrine](docs/doctrine.md).

## Current Status
We are currently in Phase 1: Validating the constraints and scaffolding the `bs-edge-agent` repository for the MT-3000 target. No production code has been released yet.

## Roadmap
For the future progression from Phase 1 through the implementation of advanced obfuscation profiles and cooperative telemetry meshes, see the [Roadmap](docs/roadmap.md).
