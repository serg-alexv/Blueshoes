# MECHA Execution Harness

**MECHA (Mechanical Execution Capability Handler)** is the strictly bounded, deterministic, dry-run-first execution harness for the Blueshoes project.

It is NOT an AI agent. It is a local wrapper operated by the human principal.

## Execution Rules

1. **Dry Run by Default**: If MECHA is executed without flags, it strictly simulates command mapping and refuses network execution.
2. **Explicit Confirmation**: To execute a real payload against the router, the operator must pass `--execute` and manually confirm the unsafe action with `--confirm unsafe:<grant_id>`.
3. **No Arbitrary Shell**: MECHA uses typed action enums (e.g., `ssh_run_netcheck`), not arbitrary string parsing.
4. **No Live Secrets in Git**: Live capability grants (`*.live.json`) are strictly gitignored. Only synthetic grants are committed as examples.
5. **Redaction**: MECHA output evidence redacts the target router's IP address.

## Usage

### Dry Run (Safe)
```bash
python3 scripts/mecha_executor.py examples/mecha/synthetic-grant.json
```
or explicitly:
```bash
python3 scripts/mecha_executor.py examples/mecha/synthetic-grant.json --dry-run
```

### Live Execution (Unsafe)
To execute a live grant (e.g., `artifacts/devship/m5-smoke-test-grant.live.json`):

```bash
python3 scripts/mecha_executor.py artifacts/devship/m5-smoke-test-grant.live.json \
  --execute \
  --confirm unsafe:grant-m5-router-smoke-001
```

*(Note: Live grants must be created locally and are explicitly gitignored to prevent credential/IP leakage).*

## Action Allowlist

The `actions` array in the capability grant supports strictly these typed operations:

- `scp_binary_to_tmp`
- `ssh_run_status`
- `ssh_run_netcheck`
- `ssh_read_journal_tail`
- `ssh_collect_router_facts`

Any unknown action is immediately rejected.

## Evidence Output
All executions (dry-run or live) output deterministic JSON evidence to `artifacts/devship/mecha-evidence/<request_id>.json`. This file must be reviewed before advancing milestone stages.
