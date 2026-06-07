# Static Bounded Profile Sets

**Definition:** Let $P$ be the set of permissible routing states. $E_{prof} \in P$ guarantees deterministic behavior. The generation of unbounded dynamic routing strings evaluates to $\text{false}$.

## The Profile Matrix $P$
1. **$P_{DIRECT}$:** The baseline operational state. Obfuscation vector $= \emptyset$.
2. **$P_{DOH}$:** Evaluates routing of port 53 to an encrypted proxy. Necessary condition: Validation function $V(\text{DNS Poisoning}) = \text{true}$.
3. **$P_{ECH}$:** Drops standard SNI TLS connections, satisfying condition $C_{end\_supports\_ECH} = \text{true}$.
4. **$P_{OBF}$:** Maps affected packet streams through an AmneziaWG tunnel. Necessary condition: Validation function $V(\text{TCP RST DPI}) = \text{true}$.
