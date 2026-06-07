# Nettools Inventory

To map georestriction pathologies accurately without bloating the router, Blueshoes carefully delegates diagnostic tools across the A/C architecture.

## Router Core (`bs-edge-agent`)
These tools are native to OpenWrt, incur zero additional storage cost, and are used by the deterministic rule-engine to detect pathologies.
- **Discovery**: `ip`, `ss`, `arp` / `neigh`
- **Performance/State**: `ping`, `conntrack`
- **Routing**: `iproute2`, `nft` (or `iptables` compat)
- **Capture**: `tcpdump` (Captures pcap to RAM, immediately pushes to Workbench).
- **Canary Validation**: `curl`

## Workbench Only (`bs-workbench`)
These tools are too heavy, require significant RAM, or have large dependencies. They run externally on the Debian/RHEL VM.
- **Deep Capture Analysis**: `Wireshark`, `tshark`
- **Heavy Discovery**: `nmap`, `mtr`
- **Specialized DNS**: `kdig`, `drill` (for manual SVCB/HTTPS record validation during research).
- **Performance/Load**: `iperf3`, `h2load`

## Explicitly Forbidden in Runtime
- **MITM Proxies**: `Charles Proxy` (See `mitm-ban.md`). Never installed on the router.
- **Unbounded Scanners**: Masscan or automated nmap sweeps from the router itself.
