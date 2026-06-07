# Cooperative Lab Tools

While transparent MITM is banned in production (`mitm-ban.md`), deep diagnostic work during the research phase may require observing TLS negotiation.

## Allowed Lab Policy: Charles Proxy
Tools like **Charles Proxy** or **mitmproxy** are permitted *only* under the following strict conditions:
1. **Never on the Router**: These tools live exclusively in the `bs-workbench` or on a dedicated lab machine.
2. **Explicit Client Cooperation**: The client device (e.g., test smartphone or browser) must be explicitly configured to use the proxy (e.g., via manual proxy settings or proxy auto-config).
3. **Ephemeral CAs**: Root certificates installed for testing must be easily identifiable and wiped after the diagnostic session.

This approach ensures the system can diagnose application-level georestrictions without embedding MITM capabilities into the router runtime.
