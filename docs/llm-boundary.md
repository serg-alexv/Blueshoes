# The LLM Boundary

To prevent unpredictable system behavior, Blueshoes establishes a strict "air gap" between the Large Language Model and the router's execution environment.

## Physical Isolation
The LLM does not exist on the router. Running models like `llama.cpp` on 512MB RAM causes severe memory swapping, killing the router's ability to forward packets. The LLM lives exclusively on the external Workbench.

## Read-Only Execution
The LLM operates in a strictly read-only capacity. It parses telemetry data (connection drops, DPI signatures) and suggests a static profile to switch to. 

## No Shell Access
Under no circumstances is the LLM allowed to generate and execute shell commands (`iptables`, `sh`, `uci`) on the router. Bypassing the static profiles to let an AI write firewall rules dynamically is an unacceptable security and stability risk.
