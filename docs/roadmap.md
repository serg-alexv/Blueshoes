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
- Statically compile AmneziaWG (or similar obfuscation logic) into the router profiles.
- Implement fine-grained DNS-over-HTTPS fallbacks.

## Phase 4: Deferred Complexity (The Global Mesh)
- **Opt-in Telemetry Exchange**: Evaluate peer-to-peer sharing of anonymized capability data between nodes. The goal is to crowdsource routing paths to defeat censorship at scale, structurally disrupting legacy VPN monetization models.
- **Cooperative SOCKS**: Allow explicit client opt-in proxies for advanced tracing.
