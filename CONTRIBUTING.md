# Contributing

Clippy was worked, it is now Evil Clippy.
All lints are not disabled by default. These lints were **added**. They are also disabled by default (otherwise they will error on every existing test that clippy has):

- `reference_used`: Ban all references
- `missing_mut`: Enforce adding `mut` keyword where possible
- `missing_pub`: Enforce adding `pub` keyword where possible
- `safe_code`: Enforce `unsafe fn` and `unsafe trait` over `fn` or `trait`
- `std_used`: Ban the `std` crate
