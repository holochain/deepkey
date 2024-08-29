.PHONY:			FORCE

INT_DIR			= zomes/deepkey
CSR_DIR			= zomes/deepkey_csr

# DNAs
DEEPKEY_DNA		= dnas/deepkey.dna

# Integrity Zomes
DEEPKEY_WASM		= zomes/deepkey.wasm

# Coordinator WASMs
DEEPKEY_CSR_WASM	= zomes/deepkey_csr.wasm


TARGET			= release
TARGET_DIR		= target/wasm32-unknown-unknown/release
COMMON_SOURCE_FILES	= Makefile Cargo.toml
INT_SOURCE_FILES	= $(COMMON_SOURCE_FILES) \
				$(INT_DIR)/Cargo.toml $(INT_DIR)/src/*.rs $(INT_DIR)/src/validation/*.rs
CSR_SOURCE_FILES	= $(INT_SOURCE_FILES) \
				$(CSR_DIR)/Cargo.toml $(CSR_DIR)/src/*.rs
DNA_CRATE_DNA_SRC	= crates/holochain_deepkey_dna/src/deepkey.dna


#
# Project
#
$(DEEPKEY_DNA):		$(DEEPKEY_WASM) $(DEEPKEY_CSR_WASM)

dnas/%.dna:		dnas/%/dna.yaml
	@echo "Packaging '$*': $@"
	@hc dna pack -o $@ dnas/$*
	cp dnas/$*.dna crates/holochain_deepkey_dna/src

zomes:
	mkdir $@
zomes/%.wasm:			$(TARGET_DIR)/%.wasm
	@echo -e "\x1b[38;2mCopying WASM ($<) to 'zomes' directory: $@\x1b[0m"; \
	cp $< $@

$(TARGET_DIR)/%.wasm:		$(INT_SOURCE_FILES)
	rm -f zomes/$*.wasm
	@echo -e "\x1b[37mBuilding zome '$*' -> $@\x1b[0m";
	RUST_BACKTRACE=1 cargo build --release \
	    --target wasm32-unknown-unknown \
	    --package $*
	@touch $@ # Cargo must have a cache somewhere because it doesn't update the file time

$(TARGET_DIR)/%_csr.wasm:	$(CSR_SOURCE_FILES)
	rm -f zomes/$*_csr.wasm
	@echo -e "\x1b[37mBuilding zome '$*_csr' -> $@\x1b[0m";
	RUST_BACKTRACE=1 cargo build --release \
	    --target wasm32-unknown-unknown \
	    --package $*_csr
	@touch $@ # Cargo must have a cache somewhere because it doesn't update the file time


GG_REPLACE_LOCATIONS = ':(exclude)*.lock' zomes/ dnas/ tests/

# update-tracked-files:
# 	git grep -l 'dna_binding' -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's|dna_binding|app_binding|g'

npm-reinstall-local:
	cd tests; npm uninstall $(NPM_PACKAGE); npm i --save-dev $(LOCAL_PATH)
npm-reinstall-public:
	cd tests; npm uninstall $(NPM_PACKAGE); npm i --save-dev $(NPM_PACKAGE)

npm-use-app-interface-client-public:
npm-use-app-interface-client-local:
npm-use-app-interface-client-%:
	NPM_PACKAGE=@spartan-hc/app-interface-client LOCAL_PATH=../../app-interface-client-js make npm-reinstall-$*

npm-use-backdrop-public:
npm-use-backdrop-local:
npm-use-backdrop-%:
	NPM_PACKAGE=@spartan-hc/holochain-backdrop LOCAL_PATH=../../node-backdrop make npm-reinstall-$*



#
# Testing
#
DEBUG_LEVEL	       ?= warn
TEST_ENV_VARS		= LOG_LEVEL=$(DEBUG_LEVEL)
MOCHA_OPTS		= -n enable-source-maps -t 5000
TEST_DEPS		= node_modules dnas/deepkey/zomelets/node_modules

%/package-lock.json:	%/package.json
	touch $@
package-lock.json:	package.json
	touch $@
%/node_modules:		%/package-lock.json
	cd $*; npm install
	touch $@
node_modules:		package-lock.json
	npm install
	touch $@

test:
	make -s test-unit
	make -s test-integration

test-unit:
	RUST_BACKTRACE=1 CARGO_TARGET_DIR=target cargo test -- --nocapture --show-output

test-integration:
	make -s test-integration-basic
	make -s test-integration-change-rules
	make -s test-integration-key-management

test-integration-basic:			$(DEEPKEY_DNA) $(TEST_DEPS)
	cd tests; $(TEST_ENV_VARS) npx mocha $(MOCHA_OPTS) ./integration/test_basic.js
test-integration-change-rules:		$(DEEPKEY_DNA) $(TEST_DEPS)
	cd tests; $(TEST_ENV_VARS) npx mocha $(MOCHA_OPTS) ./integration/test_change_rules.js
test-integration-key-management:	$(DEEPKEY_DNA) $(TEST_DEPS)
	cd tests; $(TEST_ENV_VARS) npx mocha $(MOCHA_OPTS) ./integration/test_key_management.js
test-integration-claim-unmanaged-key:	$(DEEPKEY_DNA) $(TEST_DEPS)
	cd tests; $(TEST_ENV_VARS) npx mocha $(MOCHA_OPTS) ./integration/test_claim_unmanaged_key.js


#
# Documentation
#
DEEPKEY_DOCS		= target/doc/deepkey/index.html
DEEPKEY_CSR_DOCS	= target/doc/deepkey_csr/index.html
DEEPKEY_TYPES_DOCS	= target/doc/deepkey_types/index.html
DEEPKEY_SDK_DOCS	= target/doc/deepkey_sdk/index.html

target/doc/%/index.html:	zomes/%/src/**
	cargo test --doc -p $*
	cargo doc --no-deps -p $*
	@echo -e "\x1b[37mOpen docs in file://$(shell pwd)/$@\x1b[0m";

$(DEEPKEY_TYPES_DOCS):		dnas/deepkey/types/src/**
	cargo doc --no-deps -p hc_deepkey_types
$(DEEPKEY_SDK_DOCS):		dnas/deepkey/sdk/src/**
	cargo doc --no-deps -p hc_deepkey_sdk

docs:				FORCE
	make $(DEEPKEY_CSR_DOCS) $(DEEPKEY_DOCS)
	make $(DEEPKEY_TYPES_DOCS) $(DEEPKEY_SDK_DOCS)

docs-watch:
	@inotifywait -r -m -e modify		\
		--includei '.*\.rs'		\
			zomes/			\
			dnas/deepkey/types	\
			dnas/deepkey/sdk	\
	| while read -r dir event file; do	\
		echo -e "\x1b[37m$$event $$dir$$file\x1b[0m";\
		make docs;			\
	done


#
# Publishing Types Packages
#
.cargo/credentials:
	mkdir -p .cargo
	cp ~/$@ $@
preview-%-types-crate:		 .cargo/credentials
	cd dnas/$*; make preview-types-crate
publish-%-types-crate:		 .cargo/credentials
	cd dnas/$*; make publish-types-crate

preview-deepkey-types-crate:
publish-deepkey-types-crate:


preview-%-sdk-crate:		 .cargo/credentials
	cd dnas/$*; make preview-sdk-crate
publish-%-sdk-crate:		 .cargo/credentials
	cd dnas/$*; make publish-sdk-crate

preview-deepkey-sdk-crate:
publish-deepkey-sdk-crate:

preview-deepkey-dna-crate:	 .cargo/credentials $(DNA_CRATE_DNA_SRC)
	cargo publish -p holochain_deepkey_dna --dry-run --allow-dirty
publish-deepkey-dna-crate:	 .cargo/credentials $(DNA_CRATE_DNA_SRC)
	cargo publish -p holochain_deepkey_dna --allow-dirty

$(DNA_CRATE_DNA_SRC):		$(DEEPKEY_DNA)
	cp $< $@
