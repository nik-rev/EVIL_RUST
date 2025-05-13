# Evil Rust

Evil Rust is Rust where we _fully given in and embrace the unsafe_. [Inspired by Crust.](https://github.com/tsoding/Crust)

> [!CAUTION]
>
> This only exists for fun and learning purposes.
>
> Please don't write actual software using EVIL RUST.
>
> **YOU HAVE BEEN WARNED.**

## Rules of Evil Rust

1. Every function is `unsafe`.
1. `std` is **forbidden**.
1. Everything is `mut`.
1. Everything is `pub`.
1. References `&` are not allowed. Only raw, unsafe pointers are. `*mut` and `*const`.
1. Cargo is **forbidden**. Use the compiler directly instead.

These rules are automatically enforced by a set of custom clippy lints, from `evil-clippy`.

## Hello World

Create `hello_world.rs`:

```rs
#![no_std]
// in Rust, `main` needs to be "safe". That means we can't use it
#![no_main]

use core::ffi::{c_char, c_int};

unsafe extern "C" {
    pub unsafe fn printf(fmt: *const u8, ...) -> c_int;
}

#[panic_handler]
#[expect(
    clippy::reference_used,
    reason = "`#[panic_handler]` must receive reference"
)]
pub unsafe fn panic_handler(mut info: &::core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(mut argc: i32, mut argv: *mut *mut c_char) -> i32 {
    unsafe {
        printf("Hello, world!\n\0".as_ptr() as *const _);
    }
    0
}
```

Compile and run (process described below). Output:

```
Hello, world!
```

## Evil Rust program which breaks the rules is rejected

An evil Rust program which does not abide by the rules will be rejected:

```rs
#![no_main]

pub fn missing_unsafe() {}
unsafe fn missing_pub() {}
pub unsafe fn arg_could_be_mut(arg: ()) {}
pub unsafe fn references_are_banned(mut arg: &()) {}

pub unsafe extern "C" fn main(mut argc: i32, mut argv: *mut *mut core::ffi::c_char) -> i32 { 0 }
```

Errors:

```rust
error: function must be `unsafe`
 --> errors.rs:3:5
  |
3 | pub fn missing_unsafe() {}
  |     ^
  |
help: make this function unsafe: `unsafe`
 --> errors.rs:3:5
  |
3 | pub fn missing_unsafe() {}
  |     ^
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#safe_code
  = note: `-D clippy::safe-code` implied by `-D clippy::safe-fn`
  = help: to override `-D clippy::safe-fn` add `#[allow(clippy::safe_code)]`

error: item must be `pub`
 --> errors.rs:4:1
  |
4 | unsafe fn missing_pub() {}
  | ^ help: make this item public: `pub`
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#missing_pub
  = note: requested on the command line with `-D clippy::missing-pub`

error: parameter must be `mut`
 --> errors.rs:5:32
  |
5 | pub unsafe fn arg_could_be_mut(arg: ()) {}
  |                                ^ help: make this parameter mutable: `mut`
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#missing_mut
  = note: requested on the command line with `-D clippy::missing-mut`

error: references are not allowed
 --> errors.rs:6:46
  |
6 | pub unsafe fn references_are_banned(mut arg: &()) {}
  |                                              ^^^
  |
  = help: use a raw pointer instead: `*const`
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#reference_used
  = note: requested on the command line with `-D clippy::reference-used`

error: missing `#![no_std]`
  |
  = help: make this crate `#![no_std]`
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#missing_no_std
  = note: requested on the command line with `-D clippy::missing-no-std`

error: aborting due to 5 previous errors
```

## Installing `evil-clippy`

`evil-clippy` is a fork of clippy with some custom rules to enforce evil rust.

1. `git clone` this repository
1. `cd` to the `evil-clippy` directory
1. Run to install evil clippy:

   ```sh
   cargo build --release --bin clippy-driver -Zunstable-options --artifact-dir $"(rustc --print=sysroot)/bin"
   ```

1. The output of `rustup show active-toolchain` is the `TOOLCHAIN` for which you have installed `evil-clippy`
1. Compile a program by using `clippy-driver +TOOLCHAIN -C panic="abort" -C link-args=-lc <FILE>`

For example, the toolchain I got from `rustup show active-toolchain` is `+nightly-2025-05-01` and I ran the `hello_world.rs` example like this:

```sh
alias evil-clippy="clippy-driver +nightly-2025-05-01"

evil-clippy -C panic=abort -C link-args=-lc hello_world.rs
```

`evil-clippy` will both lint and compile your code.

[Official clippy documentation for install from source](https://doc.rust-lang.org/nightly/clippy/development/basics.html?highlight=clippy-driver#install-from-source)
