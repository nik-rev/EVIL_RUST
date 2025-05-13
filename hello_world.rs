#![no_std]
// in Rust, `main` needs to be "safe". That means we can't use it
#![no_main]

pub use core::ffi::{c_char, c_int};

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
