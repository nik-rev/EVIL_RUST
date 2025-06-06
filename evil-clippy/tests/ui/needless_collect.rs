#![allow(
    unused,
    clippy::needless_if,
    clippy::suspicious_map,
    clippy::iter_count,
    clippy::manual_contains
)]

use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList};

#[warn(clippy::needless_collect)]
#[allow(unused_variables, clippy::iter_cloned_collect, clippy::iter_next_slice)]
fn main() {
    let sample = [1; 5];
    let len = sample.iter().collect::<Vec<_>>().len();
    //~^ needless_collect
    if sample.iter().collect::<Vec<_>>().is_empty() {
        //~^ needless_collect
        // Empty
    }
    sample.iter().cloned().collect::<Vec<_>>().contains(&1);
    //~^ needless_collect
    // #7164 HashMap's and BTreeMap's `len` usage should not be linted
    sample.iter().map(|x| (x, x)).collect::<HashMap<_, _>>().len();
    sample.iter().map(|x| (x, x)).collect::<BTreeMap<_, _>>().len();

    sample.iter().map(|x| (x, x)).collect::<HashMap<_, _>>().is_empty();
    //~^ needless_collect
    sample.iter().map(|x| (x, x)).collect::<BTreeMap<_, _>>().is_empty();
    //~^ needless_collect

    // Notice the `HashSet`--this should not be linted
    sample.iter().collect::<HashSet<_>>().len();
    // Neither should this
    sample.iter().collect::<BTreeSet<_>>().len();

    sample.iter().collect::<LinkedList<_>>().len();
    //~^ needless_collect
    sample.iter().collect::<LinkedList<_>>().is_empty();
    //~^ needless_collect
    sample.iter().cloned().collect::<LinkedList<_>>().contains(&1);
    //~^ needless_collect
    sample.iter().collect::<LinkedList<_>>().contains(&&1);
    //~^ needless_collect

    // `BinaryHeap` doesn't have `contains` method
    sample.iter().collect::<BinaryHeap<_>>().len();
    //~^ needless_collect
    sample.iter().collect::<BinaryHeap<_>>().is_empty();
    //~^ needless_collect

    // Don't lint string from str
    let _ = ["", ""].into_iter().collect::<String>().is_empty();

    let _ = sample.iter().collect::<HashSet<_>>().is_empty();
    //~^ needless_collect
    let _ = sample.iter().collect::<HashSet<_>>().contains(&&0);
    //~^ needless_collect

    struct VecWrapper<T>(Vec<T>);
    impl<T> core::ops::Deref for VecWrapper<T> {
        type Target = Vec<T>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<T> IntoIterator for VecWrapper<T> {
        type IntoIter = <Vec<T> as IntoIterator>::IntoIter;
        type Item = <Vec<T> as IntoIterator>::Item;
        fn into_iter(self) -> Self::IntoIter {
            self.0.into_iter()
        }
    }
    impl<T> FromIterator<T> for VecWrapper<T> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            Self(Vec::from_iter(iter))
        }
    }

    let _ = sample.iter().collect::<VecWrapper<_>>().is_empty();
    //~^ needless_collect
    let _ = sample.iter().collect::<VecWrapper<_>>().contains(&&0);
    //~^ needless_collect

    #[allow(clippy::double_parens)]
    {
        Vec::<u8>::new().extend((0..10).collect::<Vec<_>>());
        //~^ needless_collect
        foo((0..10).collect::<Vec<_>>());
        //~^ needless_collect
        bar((0..10).collect::<Vec<_>>(), (0..10).collect::<Vec<_>>());
        //~^ needless_collect
        baz((0..10), (), ('a'..='z').collect::<Vec<_>>())
        //~^ needless_collect
    }

    let values = [1, 2, 3, 4];
    let mut out = vec![];
    values.iter().cloned().map(|x| out.push(x)).collect::<Vec<_>>();
    let _y = values.iter().cloned().map(|x| out.push(x)).collect::<Vec<_>>(); // this is fine

    // Don't write a warning if we call `clone()` on the iterator
    // https://github.com/rust-lang/rust-clippy/issues/13430
    let my_collection: Vec<()> = vec![()].into_iter().map(|()| {}).collect();
    let _cloned = my_collection.into_iter().clone();
    let my_collection: Vec<()> = vec![()].into_iter().map(|()| {}).collect();
    let my_iter = my_collection.into_iter();
    let _cloned = my_iter.clone();
    // Same for `as_slice()`, for same reason.
    let my_collection: Vec<()> = vec![()].into_iter().map(|()| {}).collect();
    let _sliced = my_collection.into_iter().as_slice();
    let my_collection: Vec<()> = vec![()].into_iter().map(|()| {}).collect();
    let my_iter = my_collection.into_iter();
    let _sliced = my_iter.as_slice();
    // Assignment outside of main scope
    {
        let x;
        {
            let xxx: Vec<()> = vec![()].into_iter().map(|()| {}).collect();
            x = xxx.into_iter();
            for i in x.as_slice() {}
        }
    }
}

fn foo(_: impl IntoIterator<Item = usize>) {}
fn bar<I: IntoIterator<Item = usize>>(_: Vec<usize>, _: I) {}
fn baz<I: IntoIterator<Item = usize>>(_: I, _: (), _: impl IntoIterator<Item = char>) {}

mod issue9191 {
    use std::cell::Cell;
    use std::collections::HashSet;
    use std::hash::Hash;
    use std::marker::PhantomData;
    use std::ops::Deref;

    fn captures_ref_mut(xs: Vec<i32>, mut ys: HashSet<i32>) {
        if xs.iter().map(|x| ys.remove(x)).collect::<Vec<_>>().contains(&true) {
            todo!()
        }
    }

    #[derive(Debug, Clone)]
    struct MyRef<'a>(PhantomData<&'a mut Cell<HashSet<i32>>>, *mut Cell<HashSet<i32>>);

    impl MyRef<'_> {
        fn new(target: &mut Cell<HashSet<i32>>) -> Self {
            MyRef(PhantomData, target)
        }

        fn get(&mut self) -> &mut Cell<HashSet<i32>> {
            unsafe { &mut *self.1 }
        }
    }

    fn captures_phantom(xs: Vec<i32>, mut ys: Cell<HashSet<i32>>) {
        let mut ys_ref = MyRef::new(&mut ys);
        if xs
            .iter()
            .map({
                let mut ys_ref = ys_ref.clone();
                move |x| ys_ref.get().get_mut().remove(x)
            })
            .collect::<Vec<_>>()
            .contains(&true)
        {
            todo!()
        }
    }
}

pub fn issue8055(v: impl IntoIterator<Item = i32>) -> Result<impl Iterator<Item = i32>, usize> {
    let mut zeros = 0;

    let res: Vec<_> = v
        .into_iter()
        .filter(|i| {
            if *i == 0 {
                zeros += 1
            };
            *i != 0
        })
        .collect();

    if zeros != 0 {
        return Err(zeros);
    }
    Ok(res.into_iter())
}

mod issue8055_regression {
    struct Foo<T> {
        inner: T,
        marker: core::marker::PhantomData<Self>,
    }

    impl<T: Iterator> Iterator for Foo<T> {
        type Item = T::Item;
        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next()
        }
    }

    fn foo() {
        Foo {
            inner: [].iter(),
            marker: core::marker::PhantomData,
        }
        .collect::<Vec<&i32>>()
        .len();
    }
}
