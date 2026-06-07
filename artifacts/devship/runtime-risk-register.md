# Runtime Risk Register

| Risk ID | Component | Description | Mitigation Strategy | Doctrine Alignment |
|---------|-----------|-------------|---------------------|--------------------|
| RISK-01 | Execution | `dangerous_execution` feature bypass | Double-gated runtime flags (`--unsafe-execute --confirm unsafe:<req_id>`). | Rollback is Sacred |
