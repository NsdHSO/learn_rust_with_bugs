//! Fix all generics deep dive errors so this crate compiles.
//! 
//! Learn: Trait bounds, where clauses, PhantomData, const generics
//!
//! Each error has ERROR comment and HINT.

fn main() {
    // ===========================================================================
    // ERROR 1: missing trait bounds
    // ===========================================================================
    // HINT: Need PartialOrd to compare. Add bound.
    fn largest<T>(list: &[T]) -> &T { // ERROR: T has no Ord
        list.iter().max().unwrap()
    }

    // ===========================================================================
    // ERROR 2: multiple bounds
    // ===========================================================================
    // HINT: Use where clauses for clarity.
    fn process<T: Clone + Debug>(t: T) { // OK but verbose
        // ...
    }

    // ===========================================================================
    // ERROR 3: associated types vs generics
    // ===========================================================================
    // HINT: Associated types: one impl output. Generics: multiple outputs possible.
    trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }
    // vs
    trait GenIterator<T> {
        fn next(&mut self) -> Option<T>;
    }

    // ===========================================================================
    // ERROR 4: PhantomData for variance
    // ===========================================================================
    use std::marker::PhantomData;
    struct MyStruct<T> {
        _marker: PhantomData<T>,
        ptr: *const T, // raw pointer - variance matters
    }

    // ===========================================================================
    // ERROR 5: const generics
    // ===========================================================================
    // HINT: const generics for compile-time size.
    fn buffer<T, const N: usize>() -> [T; N] {
        [unsafe { std::mem::zeroed() }; N]
    }

    // ===========================================================================
    // ERROR 6: generic return type
    // ===========================================================================
    // HINT: Use impl Trait for opaque return.
    // fn create() -> impl Trait { ... }
}