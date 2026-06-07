# Rhea Tribunal Adapter (T0 Passive Mode)

## Purpose
This document defines the strictly bounded "Passive Tribunal Gate" (T0) architecture. It replaces the rejected execution-heavy tribunal model.

## Doctrine
1. **No Runtime Enforcement**: The Tribunal is an advisory governance structure. It CANNOT block Git pushes, trigger CI/CD pipelines, execute tests, or modify router state.
2. **Static Templates**: All canonical tribunal schemas exist purely as static `.md` templates located in `docs/tribunal/templates/`.
3. **Ephemeral Working Area**: Active reviews occur in local development environments. Artifacts may be generated locally into `.tasks/` but MUST remain strictly git-untracked.
4. **Information Only**: Tribunal verdicts are non-binding technical advisory signals meant for human operators.

## Workflow
1. The developer fills out a `tribunal-request.md` and saves it locally.
2. The developer or local agent processes the request and emits a `tribunal-verdict.md`.
3. The human operator reads the verdict.
4. If approved by the operator, the physical implementation proceeds as normal. The tribunal artifact itself is discarded or kept locally.
