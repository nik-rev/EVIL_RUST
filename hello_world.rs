#![no_std]
// in Rust, `main` needs to be "safe". That means we can't use it
#![no_main]

use core::ffi::{c_char, c_int};

unsafe extern "C" {
    pub unsafe fn printf(fmt: *const u8, ...) -> c_int;
}

// Need to provide a custom panic handler.
// Feel free to customize it, but this is the most basic handler
//
// `#[panic_handler]` ALWAYS receives a reference to the
// `PanicInfo`. So this is the ONE time we will have to use a reference, and evil-clippy will
// be ok with this. Any OTHER references are FORBIDDEN
#[panic_handler]
pub unsafe fn panic_handler(info: &::core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: i32, argv: *mut *mut c_char) -> i32 {
    unsafe {
        printf("Hello, world!\n\0".as_ptr() as *const _);
    }
    0
}
