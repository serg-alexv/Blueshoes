# Resource Boundary Condition (MT-3000)

**Definition:** The target hardware constant $H_{target}$ is defined as MediaTek MT7981B, bounded by $RAM_{total} = 512\text{MB}$ and $Flash_{total} = 256\text{MB}$.

## Hard Thresholds
Let $E$ be the resident edge-agent process.
1. **Memory Condition (Necessary):** $RAM_{E} \le 15\text{MB}$. Failure to satisfy this inequality triggers fatal OOM under connection load $C_{high}$.
2. **Storage Condition (Necessary):** $Flash_{E} \le 5\text{MB}$. This ensures the residual variable $\Delta_{sysupgrade}$ remains mathematically positive for OpenWrt updates.
3. **CPU Condition (Necessary):** $CPU_{E_{idle}} < 1\%$.
4. **CPU Condition (Mutation):** $CPU_{E_{mut}} < 15\%$, limited to interval $t < 5\text{s}$.
5. **Database Condition (Necessary):** Size of telemetry store $D_{SQLite} \le 2\text{MB}$. Enforced via FIFO log truncation.

## Corollary
Given the aforementioned thresholds, $L \in H_{target}$ evaluates to $\text{false}$. Local LLM execution is structurally invalid. Language compilation for $E$ must strictly optimize for size (Rust `#![no_std]` or `-C opt-level=z`), rendering runtime-heavy languages (Go, Python) invalid.
