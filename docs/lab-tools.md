# Out-of-Band Lab Tools

While deep-inspection tools like Charles Proxy or mitmproxy are useful for debugging, they violate the core MITM Ban if run transparently on the router.

## Execution Rules
- These tools **must not** be installed on the router hardware.
- They may be executed on the external Workbench VM solely for lab analysis.
- If used, the client device must explicitly opt-in (e.g., by manually configuring a SOCKS proxy in their OS). We never hijack traffic transparently for deep inspection.
