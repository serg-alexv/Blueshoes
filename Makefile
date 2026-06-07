.PHONY: all build clean test

# Top-level Makefile for orchestrating the Blueshoes project

all: build

build:
	@echo "Building bs-edge-agent..."
	cd runtime/bs-edge-agent && cargo build --release

test:
	@echo "Running unit tests..."
	cd runtime/bs-edge-agent && cargo test

clean:
	@echo "Cleaning workspace..."
	cd runtime/bs-edge-agent && cargo clean

# --- The Local Message Bus (.tasks/) ---

TASKS_DIR := .tasks
PENDING   := $(TASKS_DIR)/1_pending_specs
WORKING   := $(TASKS_DIR)/2_implementing
REVIEW    := $(TASKS_DIR)/3_review_bundles
DONE      := $(TASKS_DIR)/4_completed

.PHONY: tasks-init tasks-status tasks-claim tasks-complete

tasks-init:
	@mkdir -p "$(PENDING)" "$(WORKING)" "$(REVIEW)" "$(DONE)"
	@echo "OK: initialized $(TASKS_DIR)/ pipeline"

tasks-status:
	@echo "Pending:"; ls -1 "$(PENDING)" 2>/dev/null || true
	@echo "Implementing:"; ls -1 "$(WORKING)" 2>/dev/null || true
	@echo "Review bundles:"; ls -1 "$(REVIEW)" 2>/dev/null || true
	@echo "Completed:"; ls -1 "$(DONE)" 2>/dev/null || true

# Usage: make tasks-claim SPEC=m2-journal.json
tasks-claim:
	@test -n "$(SPEC)" || (echo "ERROR: SPEC is required (e.g., make tasks-claim SPEC=m2-journal.json)"; exit 2)
	@test -f "$(PENDING)/$(SPEC)" || (echo "ERROR: missing $(PENDING)/$(SPEC)"; exit 2)
	@mv "$(PENDING)/$(SPEC)" "$(WORKING)/$(SPEC)"
	@echo "CLAIMED: $(WORKING)/$(SPEC)"
	@echo "Next: open Trae and implement strictly from that envelope; write outputs to $(REVIEW)/"

# Usage: make tasks-complete SPEC=m2-journal.json BUNDLE=m2-journal-bundle.json
tasks-complete:
	@test -n "$(SPEC)" || (echo "ERROR: SPEC is required"; exit 2)
	@test -n "$(BUNDLE)" || (echo "ERROR: BUNDLE is required"; exit 2)
	@test -f "$(WORKING)/$(SPEC)" || (echo "ERROR: missing $(WORKING)/$(SPEC)"; exit 2)
	@test -f "$(REVIEW)/$(BUNDLE)" || (echo "ERROR: missing $(REVIEW)/$(BUNDLE)"; exit 2)
	@mkdir -p "$(DONE)/$(SPEC).d"
	@mv "$(WORKING)/$(SPEC)" "$(DONE)/$(SPEC).d/"
	@mv "$(REVIEW)/$(BUNDLE)" "$(DONE)/$(SPEC).d/"
	@echo "DONE: moved SPEC and BUNDLE into $(DONE)/$(SPEC).d/"
