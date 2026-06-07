# Encrypted Client Hello (ECH) Constraint

**Definition:** Let $E_{ECH}$ be the Encrypted Client Hello payload within a TLS ClientHello struct.

## Stance
**Condition (Necessary):** $E_{ECH}$ generation is the strict mathematical responsibility of the client $C_{end}$.
**Condition (Forbidden):** $E$ (the router agent) shall not synthetically construct or inject $E_{ECH}$ on behalf of $C_{end}$.

## Router Function
**Condition (Sufficient):** $E$ observes $E_{ECH}$ transmission failure. Let failure event $F$ trigger routing profile $P_{obf}$ such that $P_{obf}(E_{ECH}) \to \text{Destination}$, preserving $E_{ECH}$ integrity without payload modification.
