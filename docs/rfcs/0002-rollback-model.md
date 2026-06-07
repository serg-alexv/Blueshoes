# The Atomic Transaction Model

Modifying router state (`iptables`, `nftables`, `ip route`) is inherently dangerous. If a script applies a bad rule, the router drops offline permanently. Blueshoes solves this by treating network configuration as an atomic transaction.

## The Execution Flow

1. **Snapshot**: Before touching anything, the agent caches the current, known-good network state into memory.
2. **Apply**: The agent injects the new fallback routing profile.
3. **Validate (Netcheck)**: The agent immediately runs a bounded connectivity check against operator-configured canary targets.
4. **Commit or Rollback**:
   - If the request succeeds within 3 seconds, the new profile is **Committed**.
   - If the request times out or fails, the transaction is aborted, and the snapshot is **Rolled Back**.

This entire sequence is bounded to a maximum of 5 seconds, ensuring the network is never left in a broken state.
