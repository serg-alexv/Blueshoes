# Network Validation (Netcheck)

The `netcheck` logic is the critical validation mechanism that determines if a routing transaction was successful or if the system must immediately roll back.

## The Validation Loop
When the `bs-edge-agent` switches a routing profile, it must immediately verify that the client has retained internet connectivity.

1. **Canary Target**: The agent dispatches a lightweight HTTP/HTTPS GET request to a known, highly available endpoint (e.g., `1.1.1.1` or a dedicated validation server).
2. **Temporal Bound**: The response must return a valid `HTTP 200 OK` within 3 seconds.
3. **Failure State**: If the request times out, returns a 5xx error, or fails DNS resolution, the validation evaluates as `FALSE`.

If `FALSE`, the agent triggers the immediate rollback function to restore the previous known-good routing table.
