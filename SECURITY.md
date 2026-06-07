# Security Policy

Blueshoes takes network integrity and operational safety extremely seriously. Because we operate at the edge router layer, we possess absolute control over LAN traffic. This authority must be mathematically constrained.

## Core Security Commitments
1. **No MITM**: Blueshoes will never install synthetic root certificates or decrypt TLS payloads.
2. **LLM Isolation**: Diagnostic LLMs are strictly read-only and operate entirely outside the router's execution environment. They cannot issue state-mutating shell commands.
3. **Data Privacy**: Telemetry exported for diagnostic routing analysis must be strictly minimized metadata (IP/port tuples, coarse timing, failure modes). Avoid collecting SNI/hostnames by default; where collection is unavoidable for debugging, it must be explicit, time-bounded, and redacted or irreversibly transformed before leaving the router.
4. **PCAP Handling**: Packet captures are high-risk artifacts. If captures are supported, they must be opt-in, time-bounded, encrypted in transit to the Workbench, and stored with short retention and least-privilege access controls.
5. **No Covert Monetization**: The project must not include hidden “VPN upsell” logic, bundled paid endpoints, affiliate defaults, or any mechanism that silently routes users through third-party commercial tunnels.

## Vulnerability Reporting
Please DO NOT report security vulnerabilities through public GitHub issues.

Instead, please email security reports to **g@timelabs.ru**. 
We will attempt to acknowledge receipt within 48 hours.

## Scope of Disclosures
We are currently in a highly experimental Phase 1. 
However, if you identify a flaw that violates our **Core Security Commitments** (e.g., an architectural design that accidentally permits TLS interception), we consider that a critical vulnerability.
