# Falsified Execution Paths (Rejected Ideas)

**Definition:** The set $F_{rejected}$ contains architectural hypotheses that have been formally evaluated and mathematically proven to violate Axioms $0 \to 4$ or Target Constraints $H_{target}$.

## Elements of $F_{rejected}$

1. **Local LLM Inference ($H_{target} \cap L$)**
   - **Evaluation:** $512\text{MB RAM} \to \text{OOM}$ for any $L_{weights} > 0.4\text{GB}$.
   - **Proof:** Swapping to NAND flash destroys $E_{obs}$ throughput. Thus, $L \in E = \text{false}$.

2. **Transparent ECH / MITM Proxying**
   - **Evaluation:** Cryptographic interception $M(T)$ mathematically negates end-to-end encryption.
   - **Proof:** Violates Axiom 2 (Cryptographic Integrity).

3. **Autonomous $L$ Mutation (AI Shell Access)**
   - **Evaluation:** Mapping $L_{suggest} \to \text{system\_call}$ bypasses deterministic bounds.
   - **Proof:** Violates Axiom 4 (Diagnostic Boundary). Nullifies Axiom 1 (Rollback Continuity) due to non-deterministic temporal validation.

4. **Containerized Sandbox ($E \in \text{Docker/LXC}$)**
   - **Evaluation:** Base kernel plus container daemon exceeds $Flash_{E} \le 5\text{MB}$.
   - **Proof:** Violates Storage Condition of $H_{target}$. Requires external USB block storage, violating "zero marginal cost" axiom.
