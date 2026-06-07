# Blueshoes Doctrine

This document serves as the absolute source of truth for the project's engineering and architectural boundaries.

## 1. Rollback is Sacred
Any mutation authority granted to `bs-edge-agent` must be deterministic, bounded, observable, and rollback-safe. If an adaptation fails to validate via a canary request within a hard timeout (e.g., 5 seconds), the agent must deterministically revert the state to the previous snapshot.

## 2. No Transparent MITM / TLS Interception
Blueshoes **forbids** TLS interception and MITM architectures. We do not generate fake certificates, we do not require users to install root CAs for production routing, and we do not transparently inspect HTTPS payloads.

## 3. Client Cooperation is Mandatory
Blueshoes relies on the client (e.g., the browser) to perform secure connection logic (like ECH). Blueshoes cannot transparently force ECH into arbitrary client TLS sessions. It can only preserve, observe, validate, and route toward ECH-capable paths.

## 4. The LLM Boundary (Read-Only Diagnostics)
An LLM may be used for diagnosing georestriction pathologies, but it is **strictly banned** from the control/mutation path.
- **Allowed:** Reading telemetry, summarizing logs, explaining signatures, generating JSON recommendations.
- **Forbidden:** Executing shell scripts on the router, dynamically modifying `iptables`/`nftables`, flashing firmware, or modifying core OpenWrt configurations.

## 5. Non-Destructive Removability
Blueshoes must be designed such that removing or stopping the `bs-edge-agent` package immediately restores the router to its vanilla, perfectly functioning state. It must fail open/closed deterministically without leaving orphaned routing rules.
