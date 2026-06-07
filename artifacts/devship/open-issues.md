# Open Issues

| ISSUE_ID | Severity | Affected Scope | Owner | Status | Required Resolution |
|----------|----------|----------------|-------|--------|---------------------|
| DEV-001 | Low | Git Tracking | AGY 2M | Open | Ensure `artifacts/devship/` is correctly tracked despite `.gitignore` rules. |
| INC-001 | CRITICAL | Security | AGY 2M | Pending Rotation | Exposed credentials (GitHub, OpenAI, OpenRouter, HF, Gemini/Vertex) in chat. Must be revoked and rotated after MT-3000 test. Tokens must not be recorded or used further. |
