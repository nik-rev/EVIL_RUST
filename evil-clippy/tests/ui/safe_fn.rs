#![no_main]
#![allow(unused)]
#![warn(clippy::safe_fn)]

fn foo_safe() {}
//~^ safe_fn
unsafe fn foo_unsafe() {}

struct A;
impl A {
    pub fn bar_safe() {}
    //~^ safe_fn
    pub unsafe fn bar_unsafe() {}
}

trait B {
    fn baz_safe() {}
    //~^ safe_fn
    unsafe fn baz_unsafe() {}

    fn quux_safe();
    //~^ safe_fn
    unsafe fn quux_unsafe();
}

#[unsafe(export_name = "main")]
pub extern "C" fn entry() {
    //~^ safe_fn
    // We don't want to lint calls
    foo_safe();
    A::bar_safe();
}
