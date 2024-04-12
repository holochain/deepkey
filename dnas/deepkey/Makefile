
NAME			= deepkey

fix-rust-compile-issue: # Force rebuild to fix rust issue (typically after dry-run)
	touch types/src/lib.rs



#
# Types package
#
preview-types-crate:
	cargo publish -p hc_$(NAME)_types --dry-run --allow-dirty
	make fix-rust-compile-issue
publish-types-crate:
	cargo publish -p hc_$(NAME)_types


#
# SDK package
#
preview-sdk-crate:
	cargo publish -p hc_$(NAME)_sdk --dry-run --allow-dirty
	make fix-rust-compile-issue
publish-sdk-crate:
	cargo publish -p hc_$(NAME)_sdk
