//! Fix all structs + lifetimes errors so this crate compiles.
//! 
//! Learn: Generic lifetimes in structs, tied_to_trait, self-ref alternatives
//!
//! Each error has ERROR comment and HINT.

fn main() {
    // ===========================================================================
    // ERROR 1: struct with multiple references
    // ===========================================================================
    // HINT: Each reference needs lifetime parameter.
    struct Pair {
        first: &str, // ERROR
        second: &str, // ERROR
    }
    let first = "hello";
    let second = "world";
    let p = Pair { first, second };

    // ===========================================================================
    // ERROR 2: generic lifetime in impl
    // ===========================================================================
    // HINT: impl must declare lifetime: impl<'a> Wrapper<'a>
    struct Wrapper<'a> { value: &'a str }
    impl Wrapper { // ERROR: missing <'a>
        fn new(v: &str) -> Wrapper { Wrapper { value: v } }
    }

    // ===========================================================================
    // ERROR 3: lifetime variance
    // ===========================================================================
    // HINT: Output lifetime is MIN of input lifetimes.
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    // ===========================================================================
    // ERROR 4: 'static as bound
    // ===========================================================================
    // HINT: T: 'static means no non-'static refs.
    fn process<T: 'static>(t: T) { // OK but restrictive
        println!("{t:?}");
    }

    // ===========================================================================
    // ERROR 5: struct with lifetime in method
    // ===========================================================================
    struct Container<'a> { items: &'a [i32] }
    impl<'a> Container<'a> {
        fn get(&self, idx: usize) -> &'a i32 { // ERROR: can't return &i32
            &self.items[idx]
        }
    }

    // ===========================================================================
    // ERROR 6: HRTB confusion
    // ===========================================================================
    // HINT: for<'a> allows any lifetime.
    // fn print_all<T: for<'a> Fn(&'a T) { ... }
}