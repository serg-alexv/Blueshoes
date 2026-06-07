# RFC 0021: Enterprise Key Rotation Architecture

## 1. Context and Problem Statement

During the Devship expansion phase, a critical incident (INC-001) highlighted the risk of manually handling API keys (GitHub, OpenAI, OpenRouter, HF, Gemini/Vertex) within prompts and the development environment.

Blueshoes requires an enterprise-level automated key rotation solution. However, this solution must respect the "Rollback is Sacred" and "Runtime First" doctrines. It cannot introduce a brittle dependency on an external cloud service in the critical path of the router's deterministic execution.

## 2. Proposed Architecture: Vault-Backed Offline-First Token Provisioning

The architecture decouples the generation and storage of secrets from their consumption by relying on short-lived, dynamically fetched credentials wherever possible.

### 2.1 Central Secret Store
- **Technology**: HashiCorp Vault or Google Secret Manager.
- **Role**: Acts as the central authority for all API keys. It handles the generation, scheduled rotation, and revocation of long-lived credentials.

### 2.2 Continuous Integration (CI/CD)
- **Technology**: GitHub Actions with Workload Identity Federation (WIF).
- **Role**: CI pipelines do *not* store long-lived service account keys. Instead, they use WIF to authenticate to the Secret Store, fetching short-lived tokens only when necessary to run advisory reviews or deploy artifacts.

### 2.3 Local Devship
- **Technology**: Application Default Credentials (ADC) or local Vault agent.
- **Role**: Developers use local tools to authenticate to the central store and inject necessary tokens into their local environment variables (`BS_OPENAI_KEY`, etc.). Tokens are **never** written to `.env` files committed to git or stored in `artifacts/devship`.

### 2.4 The Edge Device (GL-MT3000)
- **CRITICAL CONSTRAINT**: The edge device does *not* need direct access to OpenAI, OpenRouter, or HuggingFace keys. The runtime agent (`bs-edge-agent`) executes deterministic network mutations based on the offline SPEC/BUNDLE.
- **Observability Export**: If the router must upload telemetry securely (e.g., to Google Cloud Monitoring), it uses a scoped, short-lived mutual TLS (mTLS) certificate or a device-specific HMAC token provisioned during initial staging, rather than a broad API key.
- **Fallback**: If the central vault or observability endpoint is unreachable, the router falls back to its last known good routing state rather than failing closed.

## 3. Security Properties
- **No Long-Lived Keys in CI**: WIF eliminates the need to store static provider keys in GitHub Actions secrets.
- **No Keys in Prompts**: The workflow explicitly forbids providing raw credential material to LLMs or development agents.
- **Edge Isolation**: The router's critical path is insulated from the key rotation lifecycle of the governance and review tooling.

## 4. Implementation Steps (Deferred)
*This RFC outlines the target architecture. Actual implementation of Vault/WIF is deferred until the core M5 and B0 milestones are proven on hardware.*
