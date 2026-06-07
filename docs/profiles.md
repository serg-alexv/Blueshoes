# Bounded Profiles

Blueshoes uses static, pre-compiled profiles. The `bs-edge-agent` selects a profile based on deterministic pathology matching. 

## Phase 1 Profiles
1. **DIRECT**: Standard default routing. No obfuscation.
2. **DOH_ONLY**: Forces all port 53 traffic through a local DNS-over-HTTPS proxy. (Used if DNS poisoning is detected).
3. **ECH_FORCED**: Drops standard SNI TLS connections and forces the client to use ECH (Requires client support).
4. **OBFUSCATED_TUNNEL**: Routes affected traffic through an AmneziaWG (obfuscated WireGuard) tunnel. (Used if DPI TCP RSTs are detected).

Profiles are strictly bounded. There is no dynamically generated bash scripting for routing rules.
