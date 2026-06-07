# Encrypted Client Hello (ECH)

Blueshoes aims to protect user privacy without breaking internet standards. 

## Our Position on ECH
1. **Client Responsibility**: Generating the Encrypted Client Hello (ECH) is the responsibility of the client browser or OS. 
2. **No Injection**: The router agent will not synthetically construct or inject ECH headers into plaintext ClientHello packets on the fly.
3. **Preservation**: The router will observe and preserve ECH where it naturally occurs. Blueshoes will not claim to “turn on ECH” via traffic manipulation.
4. **Failure Handling**: If ECH-related handshakes appear to be systematically failing, Blueshoes will prefer non-invasive mitigations (e.g., safer DNS resolution paths that do not strip HTTPS/SVCB records). If tunneling is used at all, it must be an explicit, operator-configured, lawful egress endpoint and never a bundled commercial VPN default.
5. **No Spoofing**: Blueshoes will not attempt to spoof ECH configurations to circumvent or simulate privacy features that the client has not explicitly negotiated.
