#![no_main]
#![allow(unused)]
#![warn(clippy::safe_code)]

trait TraitSafe {}
//~^ safe_code
unsafe trait TraitUnsafe {}

fn foo_safe() {}
//~^ safe_code
unsafe fn foo_unsafe() {}

struct A;
impl A {
    pub fn bar_safe() {}
    //~^ safe_code
    pub unsafe fn bar_unsafe() {}
}

trait B {
    //~^ safe_code
    fn baz_safe() {}
    //~^ safe_code
    unsafe fn baz_unsafe() {}

    fn quux_safe();
    //~^ safe_code
    unsafe fn quux_unsafe();
}

#[unsafe(export_name = "main")]
pub extern "C" fn entry() {
    //~^ safe_code
    // We don't want to lint calls
    foo_safe();
    A::bar_safe();
}
