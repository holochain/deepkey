
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
COMMON_SOURCE_FILES	= Makefile Cargo.toml \
				dnas/deepkey/types/Cargo.toml dnas/deepkey/types/src/*.rs
INT_SOURCE_FILES	= $(COMMON_SOURCE_FILES) \
				$(INT_DIR)/Cargo.toml $(INT_DIR)/src/*.rs $(INT_DIR)/src/validation/*.rs
CSR_SOURCE_FILES	= $(INT_SOURCE_FILES) \
				$(CSR_DIR)/Cargo.toml $(CSR_DIR)/src/*.rs \
				dnas/deepkey/sdk/Cargo.toml dnas/deepkey/sdk/src/*.rs


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


#
# Testing
#
DEBUG_LEVEL	       ?= warn
TEST_ENV_VARS		= LOG_LEVEL=$(DEBUG_LEVEL)

test:			$(DEEPKEY_DNA)
	make -s test-integration

test-integration:	$(DEEPKEY_DNA)
	make -s test-basic
	make -s test-change-rules
	make -s test-key-management

test-basic:		$(DEEPKEY_DNA)
	cd tests; $(TEST_ENV_VARS) npx mocha ./integration/test_basic.js
test-change-rules:	$(DEEPKEY_DNA)
	cd tests; $(TEST_ENV_VARS) npx mocha ./integration/test_change_rules.js
test-key-management:	$(DEEPKEY_DNA)
	cd tests; $(TEST_ENV_VARS) npx mocha ./integration/test_key_management.js


#
# Documentation
#
target/doc/%/index.html:	zomes/%/src/**
	cargo test --doc -p $*
	cargo doc -p $*
	@echo -e "\x1b[37mOpen docs in file://$(shell pwd)/$@\x1b[0m";


DEEPKEY_CSR_DOCS	= target/doc/deepkey_csr/index.html

docs:			$(DEEPKEY_CSR_DOCS)
docs-watch:
	@inotifywait -r -m -e modify		\
		--includei '.*\.rs'		\
			zomes/			\
	| while read -r dir event file; do	\
		echo -e "\x1b[37m$$event $$dir$$file\x1b[0m";\
		make docs;			\
	done


#
# Publishing Types Packages
#
.cargo/credentials:
	cp ~/$@ $@
preview-%-types-crate:		 test .cargo/credentials
	cd dnas/$*; make preview-types-crate
publish-%-types-crate:		 test .cargo/credentials
	cd dnas/$*; make publish-types-crate

preview-deepkey-types-crate:
publish-deepkey-types-crate:


preview-%-sdk-crate:		 test .cargo/credentials
	cd dnas/$*; make preview-sdk-crate
publish-%-sdk-crate:		 test .cargo/credentials
	cd dnas/$*; make publish-sdk-crate

preview-deepkey-sdk-crate:
publish-deepkey-sdk-crate:
