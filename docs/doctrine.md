# Core Doctrine

Blueshoes operates on a strict set of foundational rules. These ensure the system remains safe, predictable, and fully recoverable.

## 1. The Core Goal
Maximize internet access availability globally while minimizing per-node access cost. We achieve this via deterministic routing adaptation.

## 2. Rollback Continuity
If a new routing profile breaks internet access, the router must revert to the previous known-good state within 5 seconds. The user should never be left permanently offline due to a failed obfuscation attempt.

## 3. Cryptographic Integrity
Transparent Man-In-The-Middle (MITM) architecture is forbidden. Blueshoes will never decrypt, inspect, or modify TLS payloads, nor will it require synthetic root certificates.

## 4. Diagnostic Boundary
Large Language Models (LLMs) are used strictly for offline log analysis and profile recommendations. They run on external hardware (the Workbench) and have **zero authority** to issue state-mutating shell commands on the router itself.

## 5. Non-Destructive Removability
Blueshoes must operate cleanly. Disabling or removing the Blueshoes agent must perfectly restore the vanilla OpenWrt routing configuration without leaving residual broken states behind.
