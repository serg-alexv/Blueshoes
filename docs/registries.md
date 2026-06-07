# Blueshoes Research Registries

This document maintains the 14 required research registries for long-context continuity, preventing architecture drift, and tracking operational constraints. 

## 1. Constraint Registry
- **C1: Bounded Autonomy:** AI systems may not execute arbitrary shell commands for mutation. All mutation must be mapped to statically compiled profiles.
- **C2: Local-First (Deterministic):** Core diagnostic and mutation logic runs locally on the router via static rules, independently of cloud API availability.
- **C3: Sub-5s Rollback:** Any mutation transaction must revert within 5 seconds if a canary validation fails.
- **C4: Memory Safety & Size:** The edge agent must be memory safe (Rust) while remaining under 3MB binary size to fit MT3000 flash limits.
- **C5: Hard Resource Budget:** The `bs-edge-agent` must not exceed 15MB RAM or 5MB Flash.
- **C6: Absolute LLM Boundary:** LLMs are strictly forbidden from running on the router or participating directly in the mutation control path. They are read-only and external (`bs-workbench`).
- **C7: No Transparent MITM:** TLS interception and fake root CAs are explicitly banned.

## 2. Assumption Registry
- **A1:** It is assumed that DPI mechanisms (e.g., in highly restricted regimes) block protocols based on fingerprinting (like standard WireGuard) rather than broadly blocking all UDP traffic.
- **A2:** ECH deployment on major CDNs is widespread enough that falling back to ECH (via SVCB) is a viable profile for obfuscating SNI.
- **A3:** Local SQLite is sufficient for the Observability Engine's telemetry DB, provided aggressive log rotation bounds it to < 2MB.

## 3. Failure Registry
- *(Empty - Awaiting prototype testing)*

## 4. Rollback Risk Registry
- **RR1:** Modifying local routing tables (e.g., `ip route` or macOS `networksetup`) can leave the host entirely disconnected if the daemon crashes mid-transaction. Mitigation: The daemon must use a network namespace (Linux) or strict `tun` interface routing that disappears when the process dies, failing open/closed deterministically.
- **RR2:** DNS resolver mutation. Overwriting `/etc/resolv.conf` is highly risky. Mitigation: Use local DNS proxy instead of modifying system files.

## 5. Complexity Budget Registry
- **Budget Item 1:** Avoid implementing a custom cryptographic tunnel. Use existing well-vetted libraries (e.g., standard WireGuard user-space Go library, or AmneziaWG forks).
- **Budget Item 2:** Avoid distributed state synchronization. State remains strictly local per node.
- **Budget Item 3:** Avoid containerization overhead (LXC/Docker) on the router for Phase 1. `bs-edge-agent` must be bare-metal OpenWrt native.

## 6. Terminology Registry
- **Mutation Transaction:** An atomic change to the network profile (e.g., standard -> obfuscated) followed by a verification step.
- **Canary:** A synthetic packet or request sent to a known, highly available endpoint to verify connection viability.
- **Georestriction Pathology:** The specific technical signature of a block (e.g., `TCP RST` post-SNI transmission, DNS poisoning).
- **bs-edge-agent:** The deterministic runtime daemon running on the router.
- **bs-workbench:** The external environment hosting advanced analytics and read-only LLM logic.

## 7. Platform Compatibility Registry
- **GL.iNet GL-MT3000 (OpenWrt target):** Primary Phase 1 target. MediaTek MT7981B, 512MB RAM, 256MB Flash.
- **Linux:** General support for `tun/tap` and network namespaces.
- **macOS/Windows:** Deferred for Phase 1.

## 8. Runtime Strategy Registry
- **SOCKS5 Gateway:** Easiest initial prototype strategy for application-level routing.
- **Tun2Socks / Tun Device:** Better for whole-system routing, but higher rollback risk. Prototype will likely start with SOCKS5.

## 9. Networking Strategy Registry
- **Fallback Hierarchy:** Standard -> DoH/DoT -> ECH Forced -> Obfuscated VPN (AmneziaWG) -> Tor/Mixnet (deferred).

## 10. Observability Registry
- **Local SQLite Telemetry:** Stores connection attempt, TTFB (Time To First Byte), DNS resolution time, and failure signatures.

## 11. Georestriction Registry
- **GR1: SNI Filtering:** Widespread. Mitigation: ECH.
- **GR2: DNS Poisoning:** Widespread. Mitigation: DoH/DoT.
- **GR3: Protocol Fingerprinting (WireGuard):** Widespread. Mitigation: AmneziaWG / UDP Obfuscation.

## 12. Capability Fragmentation Registry
- **CF1:** ECH relies on client support AND server/CDN support. If the CDN doesn't support it, ECH fails.

## 13. Rejected Ideas Registry
- **RI1: Fully Autonomous AI Network Scripts:** Rejected due to C1 (Bounded Autonomy). An LLM writing iptables rules dynamically is a catastrophic rollback risk.

## 14. Deferred Complexity Registry
- **DC1:** Multi-node mesh networking.
- **DC2:** Windows OS Support.
- **DC3:** Automated IP reputation scanning.
