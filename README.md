# NOTE: This is a work in progress. Check back later :P

# EVIL RUST

EVIL RUST is Rust where we _fully given in and embrace the unsafe_. Write Rust like it is C by following a list of rules enforced by `evil-clippy`. [Inspired by Crust.](https://github.com/tsoding/Crust)

> [!CAUTION]
>
> This only exists for fun and learning purposes.
>
> Please don't write actual software using EVIL RUST.
>
> **YOU HAVE BEEN WARNED.**

## Rules of EVIL RUST

1. Every function is `unsafe`.
1. Cargo is **forbidden**. Use `rustc` directly instead.
1. `std` and any crates are **forbidden**.
1. Everything is `mut`.
1. Everything is `pub`.
1. References `&` are not allowed. Only raw, unsafe pointers are. `*mut` and `*const`.
1. Rustfmt, miri or any kind of tooling that helps you write "idiomatic" or "safe" rust is **completely forbidden**.

## Evil Clippy

Automated tooling like Clippy has come a long way to allow you to write programs in Rust that are both safe, bug-free and idiomatic. But who said these rules can't also do the opposite?

Clippy was forked as `evil-clippy`. We wrote a set of custom lints to enforce maximum unsafety and guarantee that EVIL RUST rules are followed properly.

## Hello World

Here is "Hello, World!" in EVIL RUST!

Create `main.rs`:

```rs
#![no_std]
// in Rust, `main` needs to be "safe". That means we can't use it
#![no_main]

// Need to provide a custom panic handler.
// Feel free to customize it, but this is the most basic handler
#[panic_handler]
// Unfortunately, `#[panic_handler]` ALWAYS receives a reference to the
// `PanicInfo`. So this is the ONE time we will have to use a reference, and evil-clippy will
// be ok with this. Any OTHER references are FORBIDDEN
pub unsafe fn panic_handler(info: &::core::panic::PanicInfo) -> ! {
    loop {}
}

// Entry point of an Evil Rust program
#[unsafe(export_name = "main")]
pub unsafe extern "C" fn entry(argc: i32, argv: *mut *mut c_char) -> i32 {
    // TODO: Write the actual hello world
}
```

To compile the code, run the following command:

```sh
rustc
  --edition 2024
  -C debuginfo=2
  -C opt-level=z
  -C panic="abort"
  main.rs
  && evil-clippy main.rs
```

You will see:

```
Hello, world!
```

> [!CAUTION]
>
> Expect lots, lots of undefined behavior. Terrible things _will_ happen. Your hard-drive might even get re-formatted. [The rustonomicon is a recommended read](https://doc.rust-lang.org/nomicon/intro.html).
>
> **IT IS _YOUR_ RESPONSIBILITY TO ENSURE CORRECTNESS OF YOUR PROGRAM.**

> [!CAUTION]
>
> THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
> IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
> FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
> AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
> LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
> OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
> SOFTWARE.
>
> **NO GUARANTEES OF ANY KIND ARE PROVIDED.**

> [!CAUTION]
>
> Programs written in EVIL RUST may invoke **undefined behavior**, introduce **security vulnerabilities**, or cause **memory corruption**.
>
> **IT IS YOUR SOLE RESPONSIBILITY TO AUDIT AND VERIFY CORRECTNESS MANUALLY.**
