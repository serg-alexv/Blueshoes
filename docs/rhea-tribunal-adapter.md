# T0: Rhea Tribunal Adapter (Passive Gate)

This document establishes the initial T0 skeleton for the "Rhea Bounded Tribunal". Unlike a runtime governance enforcement engine, the T0 Tribunal serves as a purely advisory risk annotation layer.

## Philosophy
The T0 Tribunal provides human-in-the-loop reviewers with external, role-based risk assessments on high-risk engineering artifacts prior to deployment or explicit mutation. The Tribunal does not execute commands, does not block pipelines deterministically, and never substitutes for human authorization. 

Tribunal artifacts MUST NOT be consumed by:
- CI/CD gates
- commit logic
- push logic
- runtime authorization
- router execution paths

Tribunal outputs are informational annotations only and possess ZERO execution authority over:
- git operations
- runtime execution
- router state
- CI/CD state
- deployment flow
- human operator decisions

No persistent tribunal memory or behavioral learning exists in T0. T0 MUST NOT implement reviewer reputation, weighted trust, historical behavioral learning, persistent scoring systems, or adaptive reviewer state. Each review artifact is isolated, non-authoritative, and non-learning.

## Constraints & Anti-Patterns
To prevent the "bureaucratic recursion disease" and institutionalized hallucination, the Tribunal operates under the following hard constraints:

1. **Advisory Only**: Output is restricted to `APPROVED`, `FLAGGED`, or `VETO_RECOMMENDED`. The tribunal cannot issue a hard block that prevents engineering flow.
2. **Zero Runtime Enforcements**: The Tribunal does not contain file watchers, interceptors, or runtime execution engines. It exists purely as template files and documentation.
3. **No External Critical Path Dependencies**: The system must not place external services (e.g., OpenRouter APIs) into a critical path. If the API fails, engineering continues unimpeded.
4. **Role-Based, Not Model-Based**: Reviews are conducted from defined roles (`security_reviewer`, `governance_reviewer`, `stability_reviewer`), rather than vendor-specific profiles.
5. **No Ephemeral Ledger Illusions**: `.tasks/` remains local-only and untracked. Attempting to use it for hash-chained legal custody is an anti-pattern. Real custody remains with git logs or formal external records.

## Usage (Dry-Run / Manual Workflow)
1. **Prepare Evidence**: An operator constructs a `review-request.json` containing the task UUID, mutation intent, and diff context.
2. **Consult Personas**: The JSON is submitted (manually or via a non-critical sidecar) to external LLM providers acting as specific personas.
3. **Receive Verdicts**: The personas populate a `review-verdict.json` with their findings and advisory verdict.
4. **Human Review**: The human operator reads the verdicts. If a `VETO_RECOMMENDED` is present, the human investigates. The human makes the final `commit` / `push` decision deterministically.
