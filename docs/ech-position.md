# Encrypted Client Hello (ECH) Position

Blueshoes acknowledges that **Encrypted Client Hello (ECH)** is a critical capability for bypassing Server Name Indication (SNI) based georestrictions.

## Architectural Stance

1. **No Fake ECH**: Blueshoes cannot and will not attempt to transparently "wrap" ordinary TLS traffic in ECH on behalf of the client. This is cryptographically impossible without breaking end-to-end TLS (which is banned).
2. **Client Responsibility**: The end client (browser, app) must generate the ECH payload.
3. **Router Responsibility**: The `bs-edge-agent` is responsible for:
   - **Observability**: Ensuring the network allows DNS queries for SVCB/HTTPS records to pass unpoisoned.
   - **Routing**: If a local ISP drops packets containing ECH (e.g., identifying the GREASE values or specific TLS structures), the agent detects this pathology and routes the traffic through an obfuscated profile (e.g., AmneziaWG) so the client's ECH payload reaches the destination intact.
