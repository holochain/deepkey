
INT_DIR			= dnas/deepkey/zomes/integrity/deepkey
CSR_DIR			= dnas/deepkey/zomes/coordinator/deepkey

# DNAs
DEEPKEY_DNA		= dnas/deepkey.dna

# Integrity Zomes
DEEPKEY_WASM		= zomes/deepkey_integrity.wasm

# Coordinator WASMs
DEEPKEY_CSR_WASM	= zomes/deepkey.wasm


TARGET			= release
TARGET_DIR		= target/wasm32-unknown-unknown/release
COMMON_SOURCE_FILES	= Makefile
SOURCE_FILES		= $(COMMON_SOURCE_FILES) \
				$(INT_DIR)/Cargo.toml $(INT_DIR)/src/*.rs \
				$(CSR_DIR)/Cargo.toml $(CSR_DIR)/src/*.rs


$(DEEPKEY_DNA):		$(DEEPKEY_WASM) $(DEEPKEY_CSR_WASM)

dnas/%.dna:		dnas/%/workdir/dna.yaml
	@echo "Packaging '$*': $@"
	@hc dna pack -o $@ dnas/$*/workdir

zomes:
	mkdir $@
zomes/%.wasm:			$(TARGET_DIR)/%.wasm
	@echo -e "\x1b[38;2mCopying WASM ($<) to 'zomes' directory: $@\x1b[0m"; \
	cp $< $@

$(TARGET_DIR)/%.wasm:		$(SOURCE_FILES)
	rm -f zomes/$*.wasm
	@echo -e "\x1b[37mBuilding zome '$*' -> $@\x1b[0m";
	RUST_BACKTRACE=1 cargo build --release \
	    --target wasm32-unknown-unknown \
	    --package $*
	@touch $@ # Cargo must have a cache somewhere because it doesn't update the file time


DEBUG_LEVEL	       ?= warn
TEST_ENV_VARS		= LOG_LEVEL=$(DEBUG_LEVEL)

test-integration:	$(DEEPKEY_DNA)
	cd tests; $(TEST_ENV_VARS) npx mocha ./integration/test_basic.js
