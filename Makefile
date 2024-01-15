
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
				$(INT_DIR)/Cargo.toml $(INT_DIR)/src/*.rs
CSR_SOURCE_FILES	= $(INT_SOURCE_FILES) \
				$(CSR_DIR)/Cargo.toml $(CSR_DIR)/src/*.rs


#
# Project
#
$(DEEPKEY_DNA):		$(DEEPKEY_WASM) $(DEEPKEY_CSR_WASM)

dnas/%.dna:		dnas/%/dna.yaml
	@echo "Packaging '$*': $@"
	@hc dna pack -o $@ dnas/$*

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


GG_REPLACE_LOCATIONS = ':(exclude)*.lock' zomes/

# update-tracked-files:
# 	git grep -l 'UnitEntryTypes' -- $(GG_REPLACE_LOCATIONS) | xargs sed -i 's|UnitEntryTypes|EntryTypesUnit|g'


#
# Testing
#
DEBUG_LEVEL	       ?= warn
TEST_ENV_VARS		= LOG_LEVEL=$(DEBUG_LEVEL)

test-integration:	$(DEEPKEY_DNA)
	cd tests; $(TEST_ENV_VARS) npx mocha ./integration/test_basic.js
