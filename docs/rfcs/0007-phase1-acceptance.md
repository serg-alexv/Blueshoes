# Phase 1 Acceptance Criteria

This RFC defines exactly what the first compiled binary of the `bs-edge-agent` must successfully execute to pass Phase 1. It bridges the gap between architectural doctrine and the very first pull request.

## The Binary

- **Name**: `bs-edge-agent`
- **Location**: `/usr/sbin/bs-edge-agent` (on the router)
- **Language**: Rust or Go (compiled for `aarch64_cortex-a53` targeting MT7981B)

## Acceptance Criteria

1. **Idle Memory Footprint**: When running as a daemon and polling, the process must consume `< 15MB` of RAM (Resident Set Size).
2. **Flash Footprint**: The compiled binary (stripped) must be `< 5MB`.
3. **The Rollback Loop (Dry Run)**:
   - The agent can successfully execute `nft list ruleset > /tmp/bs.rules`.
   - The agent injects a deliberate "kill switch" rule (e.g., `nft add rule inet fw4 forward drop`).
   - The validation check (`netcheck`) attempts an HTTP GET and fails.
   - The agent successfully executes `nft -f /tmp/bs.rules` and restores connectivity within 5 seconds.
4. **Telemetry Logging**: The agent successfully logs the failure event to a local SQLite database (`/tmp/bs-telemetry.db` or persistent flash depending on IO constraints) without exceeding a 2MB database size limit.

If the binary can perform this loop reliably, Phase 1 is officially complete and we have a deterministic, adaptive networking runtime.
