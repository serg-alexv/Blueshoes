# Network Validation (Netcheck)

The `netcheck` logic is the critical validation mechanism that determines if a routing transaction was successful or if the system must immediately roll back.

## The Validation Loop
When the `bs-edge-agent` switches a routing profile, it must immediately verify that the client has retained internet connectivity.

1. **Canary Targets**: The agent dispatches lightweight HTTP/HTTPS requests to a small set of operator-configured endpoints. Avoid hard-coding single third-party IPs to reduce privacy leakage and false negatives under censorship.
2. **Temporal Bound**: Each request must return a valid `HTTP 200 OK` within 3 seconds, and the overall validation must remain bounded.
3. **Failure State**: If all canaries time out, return 5xx, or fail DNS resolution, the validation evaluates as `FALSE`.

If `FALSE`, the agent triggers the immediate rollback function to restore the previous known-good routing table.
