//! Fix all trait objects errors so this crate compiles.
//! 
//! Learn: dyn Trait, vtable, object safety, trait objects
//!
//! Each error has ERROR comment and HINT.

fn main() {
    // ===========================================================================
    // ERROR 1: generic method not object-safe
    // ===========================================================================
    trait Processor {
        fn process(&self);
        fn process_generic<T>(&self, t: T) where T: std::fmt::Debug {}
    }
    impl Processor for MyProc {
        fn process(&self) {}
        fn process_generic<T: std::fmt::Debug>(&self, _t: T) {}
    }
    let p: Box<dyn Processor> = Box::new(MyProc);
    p.process_generic(5); // ERROR: generic not in vtable

    // ===========================================================================
    // ERROR 2: returning Self from trait object
    // ===========================================================================
    trait Factory {
        fn create() -> Self;
    }
    let f: Box<dyn Factory> = Box::new(MyFactory);
    let obj = f.create(); // ERROR: Self not object-safe in dyn

    // ===========================================================================
    // ERROR 3: &dyn vs Box<dyn>
    // ===========================================================================
    fn display(d: &dyn std::fmt::Display) {
        println!("{d}");
    }
    let x = 5;
    display(&x); // OK

    // ===========================================================================
    // ERROR 4: empty trait not object-safe
    // ===========================================================================
    trait Empty {}
    let e: Box<dyn Empty> = Box::new(EmptyImpl); // ERROR: empty trait

    // ===========================================================================
    // ERROR 5: trait with no methods
    // ===========================================================================
    trait Marker {}
    let m: Box<dyn Marker> = Box::new(MarkerImpl); // ERROR

    // ===========================================================================
    // ERROR 6: returning &dyn Trait
    // ===========================================================================
    // fn get_trait() -> &dyn MyTrait { ... } // ERROR: need object-safe return

    // ===========================================================================
    // ERROR 7: slicing trait objects
    // ===========================================================================
    let items: Vec<Box<dyn std::fmt::Display>> = vec![];
    let slice: &[Box<dyn std::fmt::Display>] = &items; // ERROR: DST
}

struct MyProc;
struct MyFactory;
struct EmptyImpl;
struct MarkerImpl;
impl Empty for EmptyImpl {}
impl Marker for MarkerImpl {}
impl Factory for MyFactory {
    fn create() -> Self { MyFactory }
}