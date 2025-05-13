# Contributing

Clippy was forked, it is now Evil Clippy!

- All **new** lints were set to `allow` by default.
- Our custom _evil_ lints are `deny` by default.
- These **by-default** are force-passed as arguments in the [`driver.rs`](evil-clippy/src/driver.rs) file.

- [`reference_used`][reference_used]: Ban all references
- [`missing_mut`][missing_mut]: Enforce adding `mut` keyword where possible
- [`missing_pub`][missing_pub]: Enforce adding `pub` keyword where possible
- [`safe_code`][safe_code]: Enforce `unsafe fn` and `unsafe trait` over `fn` or `trait`
- [`std_used`][std_used]: Ban the `std` crate

[reference_used]: evil-clippy/clippy_lints/src/reference_used.rs
[missing_mut]: evil-clippy/clippy_lints/src/missing_mut.rs
[missing_pub]: evil-clippy/clippy_lints/src/missing_pub.rs
[safe_code]: evil-clippy/clippy_lints/src/safe_code.rs
[std_used]: evil-clippy/clippy_lints/src/std_used.rs
