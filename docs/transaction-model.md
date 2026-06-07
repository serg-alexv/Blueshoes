# Transaction Model

The core of Blueshoes is the **Rollback-Safe Transaction Model**. Networking mutation is dangerous; applying a bad `nftables` or routing rule can permanently disconnect the device.

## The Atomic Transaction
Every time the `bs-edge-agent` changes a profile, it executes a transaction:

1. **Snapshot**: The current known-good routing state is cached in RAM.
2. **Apply**: The new profile rules are injected into `nftables`/`iproute2`.
3. **Validate**: The agent immediately fires a Canary Request (e.g., `curl --connect-timeout 3 https://validation-endpoint.com`).
4. **Commit or Rollback**:
   - If the canary succeeds (HTTP 200 OK), the snapshot is discarded.
   - If the canary times out or fails, the transaction aborts, and the snapshot is restored *immediately* (< 5 seconds total).

This guarantees that the router never stays offline due to a bad automated profile swap.
