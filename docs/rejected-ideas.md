# Rejected Architectural Ideas

To prevent repeating mistakes, we document ideas that have been formally evaluated and rejected for Phase 1.

## 1. Local LLM on the MT-3000
**Rejected**: 512MB RAM cannot support meaningful LLM inference without heavily swapping to flash memory, destroying the router's base packet-forwarding capabilities.

## 2. Transparent ECH / TLS MITM
**Rejected**: Cryptographically impossible to do transparently without breaking end-to-end encryption. Violates our core security doctrine.

## 3. Autonomous AI Shell Execution
**Rejected**: Allowing an LLM to write `iptables` rules directly is a massive security and rollback risk. All mutation must go through deterministic, pre-compiled profiles.

## 4. Containerized Router Sandbox (LXC/Docker)
**Rejected**: Base container kernels require too much flash memory (>5MB). It would require an external USB drive, complicating the installation and violating the zero-marginal-cost philosophy.
