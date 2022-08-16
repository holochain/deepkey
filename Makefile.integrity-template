##
# Test and build hc-zomes Project
#
# This Makefile is primarily instructional; you can simply enter the Nix environment for
# holochain development (supplied by holonix;) via `nix-shell` and run
# `make test` directly, or build a target directly, eg. `nix-build -A hc-zomes`.
#
SHELL		= bash
DNANAME		= integrity-template
HAPP		= ./dna/workdir/happ/$(DNANAME).happ
DNA		= ./dna/workdir/dna/$(DNANAME).dna
WASM		= ./target/wasm32-unknown-unknown/release/integrity.wasm \
		  ./target/wasm32-unknown-unknown/release/coordinator.wasm

# External targets; Uses a nix-shell environment to obtain Holochain runtimes, run tests, etc.
.PHONY: all FORCE
all: nix-test

# nix-test, nix-install, ...
nix-%:
	nix-shell --pure --run "make $*"

# Internal targets; require a Nix environment in order to be deterministic.
# - Uses the version of `hc` on the system PATH.
# - Normally called from within a Nix environment, eg. run `nix-shell`
.PHONY:		rebuild install build
rebuild:	clean build

install:	build

build:		$(HAPP)

$(HAPP):	$(DNA)
	@echo "Packaging HAPP:"
	@hc app pack dna/workdir/happ
	@ls -l $@

# Package the DNA from the built target release WASM
$(DNA):		$(WASM) FORCE
	@echo "Packaging DNA:"
	@hc dna pack dna/workdir/dna
	@ls -l $@

# Recompile the target release WASM
$(WASM): FORCE
	@echo "Building  DNA WASM:"
	@RUST_BACKTRACE=1 CARGO_TARGET_DIR=target cargo build \
	    --release --target wasm32-unknown-unknown


#
# Testing.  Requires a Nix environment, ie: make nix-test-debug
#
.PHONY: test test-debug test-unit test-old test-old-debug test-dna test-dna-debug

test:		test-unit test-dna # test-stress # re-enable when Stress tests end reliably

test-debug:	test-unit test-dna-debug

test-unit:	FORCE
	RUST_BACKTRACE=1 cargo test \
	    -- --nocapture
#
# These @old holochain/tryorama tests don't seem to work...  Use @whi/holochain-backdrop
# 

test-old:	$(DNA) FORCE
	@echo "Starting Scenario tests in $$(pwd)..."; \
	    ( [ -d  node_modules ] || npm install ) && npm test

test-old-debug: $(DNA) FORCE
	@echo "Starting Scenario tests in $$(pwd)..."; \
	    ( [ -d  node_modules ] || npm install ) && npm run test-debug

tests/package-lock.json:	tests/package.json
	touch $@

tests/node_modules:		tests/package-lock.json
	cd tests; npm install
	touch $@

test-dna:	$(DNA) tests/node_modules FORCE
	cd tests; npx mocha integration/test_api.js

test-dna-debug:	$(DNA) tests/node_modules FORCE
	cd tests; LOG_LEVEL=silly npx mocha integration/test_api.js


# Generic targets; does not require a Nix environment
.PHONY: clean
clean:
	rm -rf \
	    dist \
	    tests/node_modules \
	    .cargo \
	    target \
	    Cargo.lock
