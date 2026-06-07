# Universal Pipeline Alignment Framework

**Version:** 1.0
**Scope:** Antigravity ⇄ ChatGPT ⇄ Trae ⇄ (optional) OpenRouter
**Core Principle:** A "boring, stupid, survivable" file-based message bus using a git-untracked `.tasks/` folder.

> [!WARNING]
> Do NOT build an active orchestrator, add external queueing, or create a CI/CD platform. This governance artifact defines a passive, strict, lock-by-move file contract.
> **ANTI-ARCHITECTURE ADDICTION RULE**: No new governance layer before new runtime evidence.

## 0. Baseline Roles

| Component | Role | Allowed | Forbidden |
| :--- | :--- | :--- | :--- |
| **ChatGPT / OpenRouter** | Architect | Writes `SPEC` to `1_pending_specs/`. | Editing repo files, pushing to git, writing shell commands. |
| **Trae** | Executor | Reads `SPEC`, modifies files within scope, writes `BUNDLE` to `3_review_bundles/`. | Pushing to git, committing secrets, writing outside scope. |
| **Antigravity** | Gatekeeper / Executor | Validates `BUNDLE` against Alignment Gate. **Bounded Delegated Execution:** AGY is permitted to execute approved local/SSH commands only from claimed SPEC envelopes and only within explicitly declared operational boundaries. | Bypassing autotests, accepting bundles without evidence, autonomous hardware execution without SPEC. |

## 1. The File-Based Message Bus
All operations happen within the local, git-ignored `.tasks/` directory:
- `.tasks/1_pending_specs/`: Specs waiting for implementation.
- `.tasks/2_implementing/`: Claimed specs (physical lock acquired).
- `.tasks/3_review_bundles/`: Trae execution outputs ready for Gatekeeper review.
- `.tasks/4_completed/`: Archived artifacts.

**Locking Rule**: Moving a file from `1_pending` to `2_implementing` acquires the lock. Exactly ONE active spec is allowed in `2_implementing` at a time.

## 2. Standard Interfaces (JSON Schemas)

### SPEC Schema (`urn:tasks:spec:v1`)
Architects MUST output this exact JSON:
```json
{
  "schema_id": "urn:tasks:spec:v1",
  "request_id": "uuid-v4",
  "created_at_utc": "RFC3339 timestamp",
  "actor": "who authored the spec",
  "repo": {
    "name": "timelabs-npo/Blueshoes",
    "branch": "main"
  },
  "intent": "implement|refactor|harden|analyze",
  "scope_paths": ["src/probes/"],
  "constraints": {
    "single_writer": true,
    "no_push": true,
    "no_secrets": true,
    "no_new_files": false,
    "git_untracked_tasks_bus": true
  },
  "requirements": ["concrete requirements"],
  "acceptance_criteria": ["verifiable outcomes"],
  "test_plan": {
    "autotest_command": "make test",
    "minimum_checks": ["must pass M1 mutation audit"]
  },
  "forbidden_actions": ["push to remote", "commit secrets"],
  "expected_artifacts": {
    "bundle_path": ".tasks/3_review_bundles/<...>.bundle.json"
  }
}
```

### BUNDLE Schema (`urn:tasks:bundle:v1`)
Executors (Trae) MUST output this exact JSON:
```json
{
  "schema_id": "urn:tasks:bundle:v1",
  "request_id": "uuid-v4 (must match SPEC)",
  "created_at_utc": "RFC3339 timestamp",
  "actor": "trae",
  "spec_ref": {
    "spec_filename": "<...>.spec.json",
    "spec_sha256": "sha256"
  },
  "changes": {
    "files_changed": ["paths"],
    "diff_unified": "full unified diff text",
    "diff_sha256": "sha256"
  },
  "execution": {
    "commands_run": ["make test"],
    "autotest": {
      "command": "make test",
      "exit_code": 0,
      "stdout": "output",
      "stderr": ""
    }
  },
  "policy_checks": {
    "scope_ok": true,
    "secrets_scan_ok": true,
    "tasks_folder_untouched_ok": true
  },
  "status": "pass|fail",
  "failure_reason": ""
}
```

## 3. The ALIGNMENT GATE
Antigravity will only execute a `git commit` and `git push` if ALL of the following conditions are met:
1. `BUNDLE.status == "pass"`
2. `BUNDLE.policy_checks` are all true.
3. Autotest `exit_code == 0`.
4. No files were changed outside `SPEC.scope_paths`.
5. The repository has no secret indicators.
6. The `.tasks/` directory remains untracked by git.

## 4. Default Autotest
Executors MUST run the local autotests before generating a bundle.
- JSON parse checks.
- `git diff --check`
- `git ls-files .tasks` (must be empty).
- Scope enforcement checks.

## 5. Explicit Prerequisite Failure & Reclassification
To prevent uncontrolled dependency-installation marathons and silent environment mutation recursion, the following rule is mandatory:

If a prerequisite fails:
1. STOP the milestone.
2. Emit a blocker/failure bundle.
3. Explicitly reclassify the task (e.g., creating a `b`-suffixed recovery milestone).
4. DO NOT continue silently mutating the environment.
