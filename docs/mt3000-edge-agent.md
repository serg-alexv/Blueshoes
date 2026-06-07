# MT-3000 Resource Budget

The target hardware for the first prototype is the GL.iNet GL-MT3000 (MediaTek MT7981B Dual-core 1.3GHz, 512MB RAM, 256MB NAND Flash).

Because preserving standard router function is paramount, `bs-edge-agent` must operate within an extremely tight resource budget.

## Hard Budgets

| Resource | Maximum Allowance | Justification |
| :--- | :--- | :--- |
| **RAM (Resident)** | 15 MB | 512MB is easily exhausted under high connection tracking loads or native OpenWrt services. |
| **Flash Storage** | 5 MB | 256MB NAND is largely consumed by the base GL.iNet/OpenWrt firmware. Leaving room for sysupgrades is critical. |
| **CPU (Idle)** | < 1% | The agent must sleep while waiting for telemetry events. |
| **CPU (Mutation)** | < 15% | Brief spikes during transaction rollback/commit are allowed. |
| **Telemetry DB** | 2 MB | Local SQLite database must aggressively rotate logs to prevent filling the flash. |

## Feature Constraints Driven by Budget

1. **No Local LLM**: Running a useful LLM requires 250MB+ RAM and large swap files. This would cripple the router. All LLM logic is moved to `bs-workbench`.
2. **Language Choice**: Go is rejected due to its 10MB+ binary footprint. The agent will be written in **Rust** using `#![no_std]` or heavy `opt-level=z` optimizations to keep the binary under 3MB.
3. **No Local VM/Containers**: Running LXC or Docker for a `bs-sandbox` is deferred, as a Debian userland exceeds the 5MB flash budget.
