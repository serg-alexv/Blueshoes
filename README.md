# Blueshoes: Rollback-Safe Local-First Adaptive Networking

Blueshoes is a long-horizon infrastructure research project focused on creating a **bounded, deterministic, and rollback-safe** adaptive networking runtime. It is specifically designed to handle georestrictions and internet capability fragmentation without resorting to unrestricted, unsafe AI autonomy.

## Target Hardware
The Phase 1 prototype specifically targets the **GL.iNet GL-MT3000 (OpenWrt)**, respecting strict 15MB RAM and 5MB Flash budgets to ensure standard routing operations are never degraded.

## Core Doctrine

- **Rollback is Sacred**: Any network mutation (routing change, protocol swap) that fails to validate within a bounded time (e.g., 5 seconds) must automatically and deterministically revert.
- **Strict A/C Architecture**: 
  - **A (`bs-edge-agent`)**: A highly constrained, native OpenWrt daemon (written in Rust) that handles telemetry and deterministic profile mutation.
  - **C (`bs-workbench`)**: An external environment that handles all heavy diagnostics and LLM reasoning.
- **The LLM Boundary**: AI is strictly read-only and advisory. It cannot mutate firewall rules, DNS, or routing dynamically.
- **No Transparent MITM**: We forbid TLS interception and root CA installation. ECH must be client-initiated.

## Documentation

- [Doctrine & Boundaries](docs/doctrine.md)
- [Architecture & Transaction Flow](docs/architecture.md)
- [MT-3000 Resource Budget](docs/mt3000-resource-budget.md)
- [Nettools Inventory](docs/nettools-inventory.md)
- [LLM / Diagnostic Boundary](docs/llm-boundary.md)
- [ECH Position](docs/ech-position.md)
- [MITM Ban Policy](docs/mitm-ban.md)
- [Lab Tools Exceptions](docs/lab-tools.md)
