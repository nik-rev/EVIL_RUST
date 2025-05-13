.SILENT:

RUSTC_ARGS = \
	--edition 2024 \
	-C debuginfo=2 \
	-C opt-level=z \
	-C panic="abort" \
	-C link-args=-lc

# These are the ONLY lints you are allowed to use
EVIL_CLIPPY_LINTS = \
	-D clippy::missing_mut \
	-D clippy::safe_fn \
	-D clippy::missing_pub \
	-D clippy::reference_used \
	-D clippy::missing_no_std
	
# Disable all default lints
#
# - Some of the default lints conflict with Evil Clippy. Such as "unused mut".
# - It's also against the rules of Evil Rust to rely on this much help from the compiler.
RUSTC_DISABLED_LINTS = \
	-A warnings \
	-A ambiguous-associated-items \
	-A arithmetic-overflow \
	-A binary-asm-labels \
	-A bindings-with-variant-name \
	-A conflicting-repr-hints \
	-A default-overrides-default-fields \
	-A elided-lifetimes-in-associated-constant \
	-A enum-intrinsics-non-enums \
	-A explicit-builtin-cfgs-in-flags \
	-A ill-formed-attribute-input \
	-A incomplete-include \
	-A ineffective-unstable-trait-impl \
	-A invalid-atomic-ordering \
	-A invalid-doc-attributes \
	-A invalid-from-utf8-unchecked \
	-A invalid-null-arguments \
	-A invalid-reference-casting \
	-A invalid-type-param-default \
	-A let-underscore-lock \
	-A long-running-const-eval \
	-A macro-expanded-macro-exports-accessed-by-absolute-paths \
	-A missing-fragment-specifier \
	-A mutable-transmutes \
	-A named-asm-labels \
	-A no-mangle-const-items \
	-A overflowing-literals \
	-A patterns-in-fns-without-body \
	-A proc-macro-derive-resolution-fallback \
	-A pub-use-of-private-extern-crate \
	-A soft-unstable \
	-A test-unstable-lint \
	-A text-direction-codepoint-in-comment \
	-A text-direction-codepoint-in-literal \
	-A unconditional-panic \
	-A undropped-manually-drops \
	-A unknown-crate-types \
	-A useless-deprecated \
 		
ARGS = $(RUSTC_ARGS) $(RUSTC_DISABLED_LINTS) $(RUSTC_ENABLED_LINTS)

compile: hello_world.rs
	clippy-driver +nightly-2025-05-01-x86_64-unknown-linux-gnu $(EVIL_CLIPPY_LINTS) $(RUSTC_DISABLED_LINTS) $(RUSTC_ARGS) hello_world.rs
