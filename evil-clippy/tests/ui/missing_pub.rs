#![warn(clippy::missing_pub)]

use core::ffi::c_int;

// should not trigger for missing_pub
unsafe extern "C" {
    pub unsafe fn printf(fmt: *const u8, ...) -> c_int;
}

fn main() {}
//~^ missing_pub
