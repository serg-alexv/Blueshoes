# Research Roadmap

## Phase 1: The A/C Separation (Current)
- Finalize constraints and documentation for GL.iNet MT-3000.
- Scaffold the `bs-edge-agent` in Rust.
- Implement the baseline Transaction Engine (Snapshot -> Apply -> Validate -> Rollback).
- Establish the SQLite telemetry database.

## Phase 2: Diagnostic Maturation
- Implement the `bs-workbench` VM.
- Integrate the Read-Only LLM for parsing telemetry dumps.
- Implement PCAP forwarding from the router to the workbench.

## Phase 3: Advanced Obfuscation Profiles
- Integrate AmneziaWG statically into the router profiles.
- Implement fine-grained DOH/ECH routing fallbacks.

## Phase 4: SOCKS/Cooperative Client Modeling
- Evaluate cooperative client routing (where clients explicitly opt-in to the `bs-edge-agent` via SOCKS5 rather than transparent routing) to improve georestriction tracing.
