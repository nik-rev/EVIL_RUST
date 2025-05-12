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

    // Function call with reference
    takes_ref(&x);
    //~^ reference_used

    // Mutable reference passed
    takes_mut_ref(&mut 42);
    //~^ reference_used

    // Raw pointers are allowed
    let raw_const: *const i32 = &x;
    let raw_mut: *mut i32 = &mut 100;
    unsafe {
        println!("{}", *raw_const);
        *raw_mut = 200;
    }

    // Closure capturing by reference
    let closure = |val: &i32| println!("{}", val);
    //~^ reference_used
    closure(&x);
    //~^ reference_used

    // Closure with mutable reference
    let mut val = 5;
    let closure_mut = |v: &mut i32| *v += 1; //~^ reference_used
    closure_mut(&mut val); //~^ reference_used

    // Struct with reference field (should trigger)
    struct RefStruct<'a> {
        r: &'a i32, //~^ reference_used
    }

    // Trait method using reference
    trait RefTrait {
        fn ref_method(&self); //~^ reference_used
    }

    struct MyStruct;
    impl RefTrait for MyStruct {
        fn ref_method(&self) {
            //~^ reference_used
            println!("Hello from ref method");
        }
    }

    let my_struct = MyStruct;
    my_struct.ref_method(); //~^ reference_used

    // Array of references
    let arr: [&i32; 2] = [&x, &42]; //~^ reference_used

    // Tuple with references
    let tup: (&i32, &mut i32) = (&x, &mut val); //~^ reference_used

    // Function pointer with reference param
    let _f: fn(&i32) = takes_ref; //~^ reference_used
}

// Function taking reference
fn takes_ref(val: &i32) {
    //~^ reference_used
    println!("{}", val);
}

// Function taking mutable reference
fn takes_mut_ref(val: &mut i32) {
    //~^ reference_used
    *val += 1;
}
