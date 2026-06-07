# Encrypted Client Hello (ECH)

Blueshoes aims to protect user privacy without breaking internet standards. 

## Our Position on ECH
1. **Client Responsibility**: Generating the Encrypted Client Hello (ECH) is the responsibility of the client browser or OS. 
2. **No Injection**: The router agent will not synthetically construct or inject ECH headers into plaintext ClientHello packets on the fly.
3. **Preservation**: The router will observe ECH traffic. If ECH traffic is being systematically dropped by the ISP, Blueshoes will route that traffic through an obfuscated tunnel to ensure the ECH payload reaches its destination intact.
