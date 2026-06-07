#!/usr/bin/env python3
import sys
import json
import datetime
import subprocess

def main():
    if len(sys.argv) < 2:
        print("Usage: mecha_executor.py <path_to_grant.json>")
        sys.exit(1)

    grant_path = sys.argv[1]
    try:
        with open(grant_path, 'r') as f:
            grant = json.load(f)
    except Exception as e:
        print(f"FAILED to parse grant: {e}")
        sys.exit(1)

    # 1. Validate Schema
    if grant.get("schema_id") != "urn:blueshoes:human-capability-grant:v1":
        print("INVALID SCHEMA ID. Refusing execution.")
        sys.exit(1)
        
    # 2. Expiration check
    expires_str = grant.get("expires_at_utc", "1970-01-01T00:00:00Z")
    try:
        # Simple ISO format parsing
        expires_str = expires_str.replace("Z", "+00:00")
        expires_dt = datetime.datetime.fromisoformat(expires_str)
        now_dt = datetime.datetime.now(datetime.timezone.utc)
        if now_dt > expires_dt:
            print(f"GRANT EXPIRED AT {expires_dt}. Refusing execution.")
            sys.exit(1)
    except Exception as e:
        print(f"FAILED to parse expiration: {e}. Refusing execution.")
        sys.exit(1)

    # 3. Target Verification
    targets = grant.get("allowed_targets", [])
    if not targets:
        print("NO TARGETS DEFINED. Refusing execution.")
        sys.exit(1)

    # Parse primary target (e.g., router:gl-mt3000:192.168.8.1)
    target_uri = targets[0]
    parts = target_uri.split(":")
    if len(parts) < 3:
        print(f"INVALID TARGET FORMAT: {target_uri}. Expected system:model:ip")
        sys.exit(1)
    target_ip = parts[2]

    # 4. Filter Configuration
    allowed_commands = grant.get("allowed_commands", [])
    forbidden_commands = grant.get("forbidden_commands", [])

    evidence = {
        "grant_id": grant.get("grant_id"),
        "executed_at_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "target_ip": target_ip,
        "results": []
    }

    print(f"=== MECHA EXECUTOR INITIALIZED ===")
    print(f"Grant ID : {grant.get('grant_id')}")
    print(f"Target   : {target_ip}")
    print(f"Commands : {len(allowed_commands)}")
    print(f"==================================")

    # 5. Execution Loop
    for cmd in allowed_commands:
        # Pre-check forbidden
        is_forbidden = False
        for fcmd in forbidden_commands:
            if fcmd in cmd:
                is_forbidden = True
                break
        
        if is_forbidden:
            res = {"command": cmd, "status": "REJECTED_BY_FORBIDDEN_LIST"}
            evidence["results"].append(res)
            print(f" [BLOCK] {cmd} -> REJECTED (matches forbidden pattern)")
            continue

        print(f" [EXEC]  {cmd}")
        
        # Translation Layer
        if cmd == "scp bs-edge-agent to /tmp/bs-edge-agent":
            # Translate logical grant to physical local command
            real_cmd = ["scp", "runtime/bs-edge-agent/target/aarch64-unknown-linux-musl/release/bs-edge-agent", f"root@{target_ip}:/tmp/bs-edge-agent"]
        else:
            # Assume command is a remote SSH execution on the router
            real_cmd = ["ssh", "-o", "ConnectTimeout=5", f"root@{target_ip}", cmd]

        try:
            out = subprocess.run(real_cmd, capture_output=True, text=True, timeout=30)
            res = {
                "command": cmd,
                "translated_cmd": " ".join(real_cmd),
                "exit_code": out.returncode,
                "stdout": out.stdout.strip(),
                "stderr": out.stderr.strip()
            }
            evidence["results"].append(res)
            
            if out.returncode != 0:
                print(f"         -> Exited {out.returncode}")
                if out.stderr.strip():
                    print(f"         -> STDERR: {out.stderr.strip()[:100]}")
            else:
                print(f"         -> OK")
                
        except subprocess.TimeoutExpired:
            res = {"command": cmd, "status": "TIMEOUT"}
            evidence["results"].append(res)
            print(f"         -> TIMEOUT (30s)")
        except Exception as e:
            res = {"command": cmd, "status": "EXCEPTION", "error": str(e)}
            evidence["results"].append(res)
            print(f"         -> EXCEPTION: {e}")

    # 6. Evidence Collection
    out_path = grant.get("evidence_output", "artifacts/devship/mecha-fallback-evidence.json")
    try:
        with open(out_path, 'w') as f:
            json.dump(evidence, f, indent=2)
        print(f"\n=== EXECUTION COMPLETE ===")
        print(f"Evidence safely persisted to: {out_path}")
    except Exception as e:
        print(f"FAILED to save evidence to {out_path}: {e}")
        print("Dumping to stdout:")
        print(json.dumps(evidence, indent=2))

if __name__ == "__main__":
    main()
