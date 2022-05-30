
SHELL			= bash

NAME			= deepkey

HAPP_BUNDLE		= DeepKey.happ
DNA_DEEPKEY		= packs/deepkey.dna

TARGET			= release
DNA_DEEPKEY_WASM	= zomes/deepkey.wasm

#
# Project
#
tests/package-lock.json:	tests/package.json
	touch $@
tests/node_modules:		tests/package-lock.json
	cd tests; npm install
	touch $@
clean:
	rm -rf \
	    tests/node_modules \
	    .cargo \
	    target \
	    zomes/target \
	    $(HAPP_BUNDLE) \
	    $(DNAREPO) \
	    $(DNA_DEEPKEY_WASM)

.PHONY: rebuild build
rebuild:			clean build
build:				$(HAPP_BUNDLE)


$(HAPP_BUNDLE):			$(DNA_DEEPKEY) packs/happ.yaml
	hc app pack -o $@ ./packs/

$(DNA_DEEPKEY):			$(DNA_DEEPKEY_WASM)

packs/%.dna:
	@echo "Packaging '$*': $@"
	@hc dna pack -o $@ packs/$*

zomes/%.wasm:			zomes/target/wasm32-unknown-unknown/release/%.wasm
	cp $< $@

zomes/target/wasm32-unknown-unknown/release/%.wasm:	Makefile zomes/%/src/*.rs zomes/%/Cargo.toml # deepkey_types/src/*.rs deepkey_types/Cargo.toml
	@echo "Building  '$*' WASM: $@"; \
	cd zomes; \
	RUST_BACKTRACE=1 CARGO_TARGET_DIR=target cargo build --release \
	    --target wasm32-unknown-unknown \
	    --package $*
	@touch $@ # Cargo must have a cache somewhere because it doesn't update the file time


crates:				deepkey_types
deep_types:			deepkey_types/src/*.rs deepkey_types/Cargo.toml
	cd $@; cargo build && touch $@


#
# Testing
#
test:				test-unit-all test-dnas
test-debug:			test-unit-all test-dnas-debug

test-unit-all:			test-unit test-unit-dna_library test-unit-happ_library test-unit-web_assets
test-unit:
	cd devhub_types;	RUST_BACKTRACE=1 cargo test
test-unit-%:
	cd zomes;		RUST_BACKTRACE=1 cargo test $* -- --nocapture

tests/test.dna:
	cp $(DNAREPO) $@
tests/test.gz:
	gzip -kc $(DNAREPO) > $@

# DNAs
test-setup:			tests/node_modules

test-dnas:			test-setup test-dnarepo		test-happs		test-webassets		test-multi
test-dnas-debug:		test-setup test-dnarepo-debug	test-happs-debug	test-webassets-debug	test-multi-debug

test-dnarepo:			test-setup $(DNAREPO)
	cd tests; RUST_LOG=none LOG_LEVEL=fatal npx mocha integration/test_dnarepo.js
test-dnarepo-debug:		test-setup $(DNAREPO)
	cd tests; RUST_LOG=info LOG_LEVEL=silly npx mocha integration/test_dnarepo.js

test-happs:			test-setup $(HAPPDNA)
	cd tests; RUST_LOG=none LOG_LEVEL=fatal npx mocha integration/test_happs.js
test-happs-debug:		test-setup $(HAPPDNA)
	cd tests; RUST_LOG=info LOG_LEVEL=silly npx mocha integration/test_happs.js

test-webassets:			test-setup $(ASSETSDNA) tests/test.gz
	cd tests; RUST_LOG=none LOG_LEVEL=fatal npx mocha integration/test_webassets.js
test-webassets-debug:		test-setup $(ASSETSDNA) tests/test.gz
	cd tests; RUST_LOG=info LOG_LEVEL=silly npx mocha integration/test_webassets.js

test-multi:			test-setup $(DNAREPO) $(HAPPDNA) $(ASSETSDNA) tests/test.gz tests/test.dna
	cd tests; RUST_LOG=none LOG_LEVEL=fatal npx mocha integration/test_multiple.js
test-multi-debug:		test-setup $(DNAREPO) $(HAPPDNA) $(ASSETSDNA) tests/test.gz tests/test.dna
	cd tests; RUST_LOG=info LOG_LEVEL=silly npx mocha integration/test_multiple.js


#
# Repository
#
clean-remove-chaff:
	@find . -name '*~' -exec rm {} \;
clean-files:		clean-remove-chaff
	git clean -nd
clean-files-force:	clean-remove-chaff
	git clean -fd
clean-files-all:	clean-remove-chaff
	git clean -ndx
clean-files-all-force:	clean-remove-chaff
	git clean -fdx
