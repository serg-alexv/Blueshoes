# Registry of Universal Constraints

**Definition:** The foundational matrix of logical constraints governing all pull requests and state changes.

## Structural Sets

1. **Hardware $H_{target}$**
   - $RAM_{max} \le 15\text{MB}$
   - $Flash_{max} \le 5\text{MB}$
2. **Language Compilation $L_{comp}$**
   - Rust (`#![no_std]` or `-C opt-level=z`)
3. **Architecture Bifurcation $\{A, C\}$**
   - $E$ (Edge, Router): Strict determinism.
   - $W$ (Workbench, External): LLM Analysis.

## Axiomatic Invariants (Theorems)

- **C1:** Rollback atomicity $t \le 5\text{s}$.
- **C2:** $E_{\text{off}} \to S_{\text{vanilla}}$ (Base routing must be restored upon exit).
- **C3:** $M(T) \ne \text{Decryption}$ (No MITM).
- **C4:** $L \notin E$ (No LLMs on the router).
- **C5:** $L \to \text{shell} = \text{false}$ (LLM cannot issue system commands).
