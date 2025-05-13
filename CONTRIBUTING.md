# Contributing

Clippy was worked, it is now Evil Clippy. All **new** lints were set to `allow` by default.
_These_ lints are `deny` by default.

- `reference_used`: Ban all references
- `missing_mut`: Enforce adding `mut` keyword where possible
- `missing_pub`: Enforce adding `pub` keyword where possible
- `safe_code`: Enforce `unsafe fn` and `unsafe trait` over `fn` or `trait`
- `std_used`: Ban the `std` crate
