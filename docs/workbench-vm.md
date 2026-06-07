# The bs-workbench VM

Because the `bs-edge-agent` must remain extremely small (under 15MB RAM), all heavy lifting is offloaded to the `bs-workbench`.

## Definition
The `bs-workbench` is an external Debian/RHEL Virtual Machine (or laptop environment) running alongside the router, either locally on the LAN or remotely.

## Responsibilities
1. **Telemetry Analysis**: Reads the SQLite logs pushed or pulled from the edge agent.
2. **PCAP Analysis**: Uses `Wireshark` or `tshark` to analyze raw packet captures.
3. **LLM Inference**: Hosts the read-only AI diagnostic brain (e.g., `llama.cpp` or API integrations) to classify novel blocking signatures.
4. **Toolchain**: Hosts heavy scanning tools like `nmap` and `h2load` that cannot fit on the router.

The workbench operates strictly out-of-band and cannot directly mutate the router's state without human approval.
