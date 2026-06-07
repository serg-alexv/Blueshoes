# Static Bounded Profiles

To maintain determinism and stability, the `bs-edge-agent` does not write firewall rules dynamically on the fly. Instead, it selects from a set of pre-compiled, statically defined profiles based on observed network failures.

## Initial Profile Set

1. **DIRECT**: The baseline state. Standard OpenWrt routing with no obfuscation.
2. **DOH_ONLY**: Routes port 53 traffic through an encrypted proxy to bypass simple DNS poisoning.
3. **ECH_STRICT**: Preserves and prioritizes connections supporting Encrypted Client Hello, dropping standard SNI leakage if necessary.
4. **OBF_TUNNEL**: Routes traffic encountering severe DPI (e.g., TCP Resets) through an obfuscated tunnel like AmneziaWG.
