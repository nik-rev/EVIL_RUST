#![warn(clippy::reference_used)]

fn main() {
    let x = 10;
    let y = &x;
    //~^ reference_used

    let z = &mut 20;
    //~^ reference_used

    let (a, ref b) = (1, 2);
    //~^ reference_used

    let c = Some(3);
    if let Some(ref d) = c {
        //~^ reference_used
        println!("{}", d);
    }

    let s = String::from("hello");
    let ref_s = &s;
    //~^ reference_used

    takes_ref(&x);
    //~^ reference_used

    takes_mut_ref(&mut 42);
    //~^ reference_used

    let raw_const: *const i32 = &x;
    //~^ reference_used
    let raw_mut: *mut i32 = &mut 100;
    //~^ reference_used
    unsafe {
        println!("{}", *raw_const);
        *raw_mut = 200;
    }

    let raw_v1 = &raw const x;
    let mut temp = 50;
    let raw_v2 = &raw mut temp;
    unsafe {
        println!("{}", *raw_v1);
        *raw_v2 = 123;
    }

    // Closure capturing by reference
    let closure = |val: &i32| println!("{}", val);
    //~^ reference_used
    closure(&x);
    //~^ reference_used

    let mut val = 5;
    let closure_mut = |v: &mut i32| *v += 1;
    //~^ reference_used
    closure_mut(&mut val);
    //~^ reference_used

    struct RefStruct<'a> {
        r: &'a i32,
        //~^ reference_used
    }

    trait RefTrait {
        fn ref_method(&self);
        //~^ reference_used
    }

    struct MyStruct;
    impl RefTrait for MyStruct {
        fn ref_method(&self) {
            //~^ reference_used
            println!("Hello from ref method");
        }
    }

    #[rustfmt::skip]
    let arr: [&i32; 2] = [
        //~^ reference_used
        &x,
        //~^ reference_used
        &42,
        //~^ reference_used
    ];

    #[rustfmt::skip]
    let tup: (
        &i32,
        //~^ reference_used
        &mut i32,
        //~^ reference_used
    ) = (
        &x,
        //~^ reference_used
        &mut val,
        //~^ reference_used
    );
}

fn takes_ref(val: &i32) {
    //~^ reference_used
    println!("{}", val);
}

fn takes_mut_ref(val: &mut i32) {
    //~^ reference_used
    *val += 1;
}
