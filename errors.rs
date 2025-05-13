#![no_main]
pub fn missing_unsafe() {}
unsafe fn missing_pub() {}
pub unsafe fn arg_could_be_mut(arg: ()) {}
pub unsafe fn references_are_banned(mut arg: &()) {}

pub unsafe extern "C" fn main(mut argc: i32, mut argv: *mut *mut core::ffi::c_char) -> i32 {
    0
}
