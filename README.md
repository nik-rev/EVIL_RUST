# Crust

Crust is Rust where we _fully given in, and embrace the unsafe_. Write Rust like it is C by following a list of rules enforced by `evil-clippy`.

> [!WARNING]
>
> This only exists for fun and learning purposes.
>
> Please don't use this for actual software.
>
> **YOU HAVE BEEN WARNED.**

> [!CAUTION]
>
> Expect lots, lots of undefined behavior. Terrible things _will_ happen. Your hard-drive might even get re-formatted. [The rustonomicon is a recommended read](https://doc.rust-lang.org/nomicon/intro.html).
>
> **IT IS _YOUR_ RESPONSIBILITY TO ENSURE CORRECTNESS OF YOUR PROGRAM.**

> [!WARNING]
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
> Programs written in Crust may invoke **undefined behavior**, introduce **security vulnerabilities**, or cause **memory corruption**.
>
> **IT IS YOUR SOLE RESPONSIBILITY TO AUDIT AND VERIFY CORRECTNESS MANUALLY.**

## Rules of Crust

1. Every function is `unsafe`.
1. Cargo is **forbidden**. Use `rustc` directly instead.
1. `std` and any crates are **forbidden**.
1. Everything is `mut`.
1. Everything is `pub`.
1. References `&` are not allowed. Only raw, unsafe pointers are. `*mut` and `*const`.
1. Rustfmt, miri or any kind of tooling that helps you write "idiomatic" or "safe" rust is **completely forbidden**.
