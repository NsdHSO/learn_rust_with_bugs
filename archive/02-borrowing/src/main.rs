//! Fix all borrowing errors so this crate compiles.

fn main() {
    // ERROR 1: two mutable borrows at the same time
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{r1} {r2}");

    // ERROR 2: mutable borrow while immutable borrows exist
    let mut t = String::from("world");
    let r3 = &t;
    let r4 = &t;
    let r5 = &mut t;
    println!("{r3} {r4} {r5}");

    // ERROR 3: mutating through a shared (immutable) reference
    let u = String::from("immutable");
    let r6 = &u;
    r6.push_str(" nope");
    println!("{r6}");
}
