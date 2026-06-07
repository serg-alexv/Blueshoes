# Diagnostic LLM Boundary

**Definition:** Let $L$ be the Large Language Model algorithm operating over a local context window. Let $E$ be the operational routing space.

## Physical Isolation
**Condition (Necessary):** $L \notin E$. The intersection of the LLM memory footprint and the router's 512MB RAM pool must be the null set $\emptyset$.

## Logical Isolation
**Condition (Necessary):** $L$ is strictly read-only relative to $E$.
**Condition (Sufficient):** $L(E_{telemetry}) \to \{report, P_{suggest}\}$.

## Forbidden Operations Set
$L$ shall never execute elements of set $F_{shell} = \{\text{iptables}, \text{nft}, \text{ip route}, \text{uci}, \text{opkg}, \text{sh}\}$.
The equation $L \to \text{shell}$ evaluates to $\text{false}$ under all operational conditions.
