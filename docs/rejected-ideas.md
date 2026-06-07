# Rejected Ideas Registry

To prevent architectural amnesia, we document ideas that have been explicitly rejected and must not be reintroduced.

## 1. Local LLM on the MT-3000
**Rejected**: 512MB RAM cannot support meaningful LLM inference (`llama.cpp` or otherwise) without heavily swapping to disk, destroying the router's base packet-forwarding capabilities.

## 2. Transparent ECH / TLS MITM
**Rejected**: Cryptographically impossible to do transparently without breaking end-to-end encryption. Violates the core doctrine of preserving privacy and security.

## 3. Autonomous AI Shell Execution
**Rejected**: Allowing an LLM to write `iptables` rules directly is a massive security and rollback risk. All mutation must go through deterministic, pre-compiled profiles.

## 4. Containerized Router Sandbox (LXC/Docker)
**Rejected for Phase 1**: Too heavy for the 256MB flash on the MT-3000. Replaced by the external `bs-workbench`.
