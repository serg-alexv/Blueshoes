# The Workbench VM

Because the `bs-edge-agent` must remain extremely small to fit on the router, all heavy analytical lifting is offloaded to the `bs-workbench`.

## What is it?
The Workbench is an external environment (e.g., a Debian Virtual Machine, a Raspberry Pi, or a developer's laptop) running alongside the router on the LAN.

## Responsibilities
1. **Telemetry Analysis**: It pulls the SQLite logs from the router to search for network pathology patterns.
2. **PCAP Analysis**: It hosts heavy tools like `Wireshark` to analyze raw packet captures.
3. **LLM Inference**: It runs the AI diagnostics that suggest new routing profiles based on the telemetry.
4. **Heavy Tooling**: It hosts active scanning tools (like `nmap` or `iperf3`) that are too large to fit in the router's 5MB flash budget.

The Workbench cannot change the router's configuration autonomously; human approval is currently required to apply its suggestions.
