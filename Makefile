
SHELL			= bash

NAME			= deepkey

HAPP_BUNDLE		= DeepKey.happ
DNA_DEEPKEY		= packs/deepkey.dna

TARGET			= release
DNA_DEEPKEY_WASM	= ./target/wasm32-unknown-unknown/release/deepkey.wasm \
			  ./target/wasm32-unknown-unknown/release/deepkey_integrity.wasm

# External targets; Uses a nix-shell environment to obtain Holochain runtimes, run tests, etc.
.PHONY: all FORCE
all:				nix-test

# nix-test, nix-install, ...
nix-%:
	nix-shell --pure --run "make $*"

#
# Project
#
clean:
	rm -rf \
	    tests/node_modules \
	    .cargo \
	    target \
	    Cargo.lock \
	    $(HAPP_BUNDLE) \
	    $(DNA_DEEPKEY)

.PHONY: rebuild build happ dna wasm
rebuild:			clean build

build:				happ

happ:				$(HAPP_BUNDLE)

$(HAPP_BUNDLE):			$(DNA_DEEPKEY) packs/happ.yaml
	hc app pack -o $@ ./packs/

dna:				$(DNA_DEEPKEY)

$(DNA_DEEPKEY):			$(DNA_DEEPKEY_WASM)

packs/%.dna:
	@echo "Packaging '$*': $@"
	@hc dna pack -o $@ packs/$*

wasm:				$(DNA_DEEPKEY_WASM)

target/wasm32-unknown-unknown/release/%.wasm:	Makefile zomes/%/Cargo.toml zomes/%/src/*.rs zomes/%/src/*/*.rs 
	@echo "Building  '$*' WASM: $@"; \
	RUST_BACKTRACE=1 CARGO_TARGET_DIR=target cargo build --release \
	    --target wasm32-unknown-unknown \
	    --package $*
	@touch $@ # Cargo must have a cache somewhere because it doesn't update the file time


crates:				deepkey_types
deep_types:			deepkey_types/src/*.rs deepkey_types/Cargo.toml
	cd $@; cargo build && touch $@


#
# Test: Building, Javascript Holochain DNA tests, Rust tests
#
test:				happ	test-dna	test-unit

test-debug:			happ	test-dna-debug	test-unit

test-unit:
	RUST_BACKTRACE=1 cargo test \
	    --verbose -j 2
	    --lib --features="mock" \
		-- --nocapture


# Test: DNAs via Holochain
tests/package-lock.json:	tests/package.json
	touch $@

tests/node_modules:		tests/package-lock.json
	cd tests; npm install
	touch $@

test-dna:			dna tests/node_modules FORCE
	cd tests; npx mocha integration/test_api.js

test-dna-debug:			dna tests/node_modules FORCE
	cd tests; LOG_LEVEL=silly npx mocha integration/test_api.js


