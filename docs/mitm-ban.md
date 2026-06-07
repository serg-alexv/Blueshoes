# The MITM Ban

Blueshoes is designed to protect network traffic, not intercept it. Therefore, any form of transparent Man-In-The-Middle (MITM) architecture is strictly forbidden.

## Rules
1. **No Decryption**: The router agent will never attempt to decrypt, inspect, or modify the payload of a TLS session.
2. **No Root Certificates**: Blueshoes will never require or prompt the user to install a synthetic Root CA on their devices.
3. **End-to-End Integrity**: Cryptographic continuity between the client and the destination server must be perfectly preserved. We route traffic; we do not read it.
