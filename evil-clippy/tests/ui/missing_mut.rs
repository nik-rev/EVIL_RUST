#![allow(unused_mut, unused, clippy::safe_fn)]
#![warn(clippy::missing_mut)]

struct Foo {
    a: (),
}

struct Bar((), (), ());

fn foo(
    a: (),
    //~^ missing_mut
    mut b: (),
    c: (),
    //~^ missing_mut
    ref d: (),
    //~^ missing_mut
    Foo { a: mut e }: Foo,
    Foo { a: f }: Foo,
    //~^ missing_mut
    (
        g,
        //~^ missing_mut
        mut h,
        i,
        //~^ missing_mut
    ): ((), (), ()),
    Bar(
        j,
        //~^ missing_mut
        mut k,
        l,
        //~^ missing_mut
    ): Bar,
) {
    let a = 4;
    //~^ missing_mut
    let mut b = 4;
}

static FOO: () = ();
//~^ missing_mut
static mut BAR: () = ();

fn main() {
    // test code goes here
}
