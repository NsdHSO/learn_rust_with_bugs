//! Fix all borrowing errors so this crate compiles.

fn main() {
    // ERROR 1: two mutable borrows at the same time
    // HINT: Rust allows only ONE &mut at a time. Introduce a scope { } so
    // r1 is dropped before r2 is created, or restructure the logic.
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{r1} {r2}");

    // ERROR 2: mutable borrow while immutable borrows exist
    // HINT: You cannot have &mut while & exists. Ensure r3 and r4 are no
    // longer used (end their scope) before taking r5.
    let mut t = String::from("world");
    let r3 = &t;
    let r4 = &t;
    let r5 = &mut t;
    println!("{r3} {r4} {r5}");

    // ERROR 3: mutating through a shared (immutable) reference
    // HINT: push_str needs &mut self. Change &u to &mut u to allow mutation.
    let u = String::from("immutable");
    let r6 = &u;
    r6.push_str(" nope");
    println!("{r6}");
}
