# Blueshoes Governance Doctrine

## 1. SYSTEM OF RECORD: Canonical Authority
- **AGY 2M** is exclusively defined as the:
  - doctrine owner
  - roadmap owner
  - SPEC/BUNDLE validator
  - synchronization verifier
  - final merge gate
  - architectural continuity engine
- **Absolute Rule**: No other model/tool may become canonical authority.

## 2. TOOLCHAIN ROLE MAP

### AGY 2M (Constitutional Orchestrator)
- **Allowed**: maintain doctrine, validate SPEC/BUNDLE, bounded command execution from approved SPEC, generate tribunal requests, aggregate review evidence, perform final acceptance review.
- **Forbidden**: silent execution, runtime mutation without explicit gates, "proceeding because constraints were clear".

### Trae (Implementation Executor)
- **Allowed**: repo edits within scope_paths, tests/builds, bundle generation.
- **Forbidden**: architectural decisions, governance changes, push without AGY gate, router mutation without SPEC.

### ChatGPT (Adversarial Reviewer)
- **Strengths**: governance drift detection, architecture critique, rollback doctrine consistency, anti-bloat enforcement.
- **Never**: repo writer, executor, runtime authority.

### GitHub Copilot (Local Coding Assistant)
- **Allowed**: autocomplete, boilerplate, tiny refactors, test scaffolding.
- **Never**: architecture authority, tribunal member, merge authority.

### OpenRouter (External Multi-Model Tribunal Transport)
- **Purpose**: parallel reviewer access.
- **Never**: runtime dependency, release blocker, router authority, automatic veto engine.

### HuggingFace (Local Reviewer Experiments)
- **Role**: model benchmarking, offline governance experiments.
- **Never**: runtime authority, router execution layer.

## 3. OPUS 4.6 — DEDICATED ROLE
- **Role**: `architecture_consistency_reviewer`
- **Specialization**: long-context doctrine drift detection, hidden governance creep identification, rollback model consistency analysis, large-system contradiction analysis, "you accidentally built a platform instead of a runtime" detection.
- **Forbidden**: suggest direct runtime mutation, approve execution, become release authority, rewrite doctrine autonomously.

### EXACT TRIGGERS FOR OPUS REVIEW
Invoke Opus ONLY when at least one condition is true:
- **Trigger Group A — Architecture Drift**: new subsystem proposed, new daemon/service proposed, new orchestration layer proposed, new watcher/interceptor proposed, CI/CD expansion proposed, external API dependency proposed.
- **Trigger Group B — Runtime Mutation**: execution gates changed, rollback logic changed, planner/executor relationship changed, dangerous_execution feature changed, watchdog semantics changed.
- **Trigger Group C — Governance Risk**: tribunal changes, SPEC/BUNDLE schema changes, AGY authority changes, push/merge logic changes, secrets/access model changes.
- **Trigger Group D — Strategic Reorientation**: changing mission, adding monetization layer, adding VPN/commercial behavior, changing MITM doctrine, changing ECH doctrine.
- **Do NOT invoke for**: typo fixes, telemetry-only probes, documentation formatting, ordinary Rust refactors, harmless tests.

## 4. STRICT DEBUG JOURNAL & DECISION LEAK PREVENTION
- **Decision Leak Prevention (HARD RULE)**: Reviewers must never consume other reviewer verdicts as truth, average each other’s scores, recursively validate consensus, or inherit authority. Every reviewer evaluates SPEC + BUNDLE + doctrine only. AGY alone aggregates to prevent recursive hallucination amplification, consensus collapse, and tribunal echo chambers.

## 5. REQUIRED REVIEW FLOW & ADVISORY GITHUB ACTIONS
- **Runtime milestone review flow**: AGY generates REVIEW_REQUEST -> human sends to reviewers -> reviewers return REVIEW_VERDICT -> AGY records evidence -> AGY summarizes -> human approves/rejects next SPEC.
- **GitHub Actions (Safe Version)**: Must be strictly "Advisory PR Review". LLM outputs post advisory findings and risk annotations to PR. Must not have merge/execution/runtime authority.

## 6. ABSOLUTE FINAL RULES
- **Runtime First**: No governance expansion without runtime evidence.
- **No External Dependency in Critical Path**: Router runtime must survive OpenRouter outage, HF outage, ChatGPT outage, AGY outage.
- **Tribunal is Advisory**: Tribunal annotates risk. Humans and deterministic tests decide.
- **AGY is Constitutional Layer**: AGY is continuity, doctrine memory, synchronization authority, gatekeeper. Everything else is replaceable.
