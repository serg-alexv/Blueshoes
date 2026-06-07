# Blueshoes Roadmap

## Phase 1: Core Engine Stabilization (Current)
- Establish the architecture and hardware constraints (GL-MT3000).
- Select the memory-safe language (Rust/Go) based on footprint testing.
- Scaffold the `bs-edge-agent` and implement the atomic Snapshot/Rollback engine.
- Initialize the local SQLite telemetry store for failure logging.

## Phase 2: Analytic Workflows
- Build the `bs-workbench` VM environment.
- Implement read-only LLM log parsing to map failures to profile suggestions.
- Set up secure PCAP forwarding from the router to the workbench.

## Phase 3: Advanced Obfuscation
- Implement fine-grained DNS-over-HTTPS/DoT fallbacks with strict privacy controls and clear user-visible configuration.
- Add support for optional, explicitly configured tunnel transports where the operator supplies and controls the egress endpoint. The project must not ship bundled commercial VPN endpoints, paid defaults, or covert monetization hooks.

## Phase 4: Deferred Complexity (The Global Mesh)
- **Opt-in Telemetry Exchange**: Evaluate peer-to-peer sharing of anonymized capability data between nodes. The goal is to crowdsource routing paths to defeat censorship at scale, structurally disrupting legacy VPN monetization models.
- **Cooperative SOCKS**: Allow explicit client opt-in proxies for advanced tracing, with strong abuse-resistance and no transparent interception.
