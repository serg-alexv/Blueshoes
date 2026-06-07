# LLM / Diagnostic Brain Boundary

To prevent the system from becoming an "unrestricted AI agent," the integration of any LLM or AI diagnostic capability is tightly constrained.

## The Physical Boundary
- **Router (`bs-edge-agent`)**: Contains **ZERO** LLM capabilities. It relies entirely on deterministic code (Rust/C) and static, embedding-free classification rules.
- **External VM (`bs-workbench`)**: Contains the LLM inference engine.

## The Logical Boundary (Read-Only)
The `bs-workbench` LLM operates under a strict **Read-Only Advisory** paradigm.

### Allowed Actions
- Ingest telemetry DB extracts (SQLite dumps) from the edge agent.
- Ingest raw PCAP files from the edge agent.
- Classify complex, novel censorship signatures (e.g., identifying a new heuristic used by a state firewall to drop UDP packets).
- Generate a human-readable JSON report.
- Recommend which statically pre-compiled profile the user should activate.

### Forbidden Actions
- The LLM **cannot** issue shell commands (`ssh`, `uci`, `opkg`) directly to the router.
- The LLM **cannot** write custom `iptables` or `nftables` rules on the fly.
- The LLM **cannot** bypass the transaction engine.

**Flow**: `Telemetry -> LLM -> JSON Recommendation -> Human Approval -> Edge Agent activates Static Profile`.
