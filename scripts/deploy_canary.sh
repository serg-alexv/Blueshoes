#!/usr/bin/env bash
set -e

ROUTER_IP=${1:-"192.168.8.1"}
ROUTER_USER=${2:-"root"}
DEST_DIR="/tmp/bs-canary"

echo "Deploying B0.1 Canary to $ROUTER_USER@$ROUTER_IP:$DEST_DIR"

# Ensure we are in the project root
cd "$(dirname "$0")/.."

# Path to the cross-compiled binaries
EDGE_AGENT="runtime/bs-edge-agent/target/aarch64-unknown-linux-musl/release/bs-edge-agent"
WATCHDOG="runtime/bs-edge-agent/target/aarch64-unknown-linux-musl/release/bs-watchdog"

if [ ! -f "$EDGE_AGENT" ] || [ ! -f "$WATCHDOG" ]; then
    echo "Error: Binaries not found! Did you run 'scripts/build_b0_release.sh' first?"
    exit 1
fi

echo "Creating tmp directory on router..."
ssh "$ROUTER_USER@$ROUTER_IP" "mkdir -p $DEST_DIR"

echo "Copying bs-edge-agent..."
scp "$EDGE_AGENT" "$ROUTER_USER@$ROUTER_IP:$DEST_DIR/"

echo "Copying bs-watchdog..."
scp "$WATCHDOG" "$ROUTER_USER@$ROUTER_IP:$DEST_DIR/"

echo "Setting executable permissions..."
ssh "$ROUTER_USER@$ROUTER_IP" "chmod +x $DEST_DIR/bs-edge-agent $DEST_DIR/bs-watchdog"

echo ""
echo "Deployment successful!"
echo ""
echo "To run the Canary MTU test, execute the following on the router:"
echo "ssh $ROUTER_USER@$ROUTER_IP '$DEST_DIR/bs-edge-agent canary'"
echo ""
echo "To view the transaction journal, run:"
echo "ssh $ROUTER_USER@$ROUTER_IP 'cat /tmp/bs-edge-journal.jsonl'"
