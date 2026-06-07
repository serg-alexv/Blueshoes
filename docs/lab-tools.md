# Out-of-Band Diagnostic Tool Constraint

**Definition:** Let $D_{lab}$ be the set of deep-inspection diagnostic tools (e.g., Charles Proxy, mitmproxy). 

## Boundary Condition
**Condition (Necessary):** The intersection $D_{lab} \cap E$ must be the null set $\emptyset$. Deep-inspection tools shall not exist on the router hardware.

## Execution Condition
**Condition (Sufficient):** $D_{lab}$ may execute exclusively within $W_{remote}$ provided that client $C_{end}$ executes an explicit, manual opt-in vector (e.g., manual SOCKS configuration) confirming mutual agreement. Ephemeral diagnostic certificates $C_{eph}$ must satisfy the condition $\text{Lifespan}(C_{eph}) \to 0$ upon session termination.
