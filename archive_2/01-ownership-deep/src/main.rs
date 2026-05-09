//! Learn: Stack vs Heap, Move semantics, Drop glue, Aliasing XOR mutability
//!
//! Each error has ERROR comment and HINT on how to fix.

fn main() {
    // ===========================================================================
    // ERROR 1: use after move
    // ===========================================================================
    // HINT: s1 was moved into print_it. Either borrow (&s1) or return ownership.
    let mut s1 = String::from("hello");
    print_it(&mut s1);
    println!("after: {s1}"); // ERROR: s1 is moved!

    // ===========================================================================
    // ERROR 2: move in tuple destructuring
    // ===========================================================================
    // HINT: s2 moved into tuple. Use borrow or clone to keep s2 alive.
    let s2 = String::from("world");
    let (_, s3) = (String::new(), &s2);
    println!("s2 = {s2}"); // ERROR: s2 moved!

    // ===========================================================================
    // ERROR 3: variable shadowing
    // ===========================================================================
    // HINT: x is i32, not String. Use different names or borrow to access original.
    let x = 5;
    let x = String::from("shadowed");
    println!("x = {}", x); // OK but lost original x

    // ===========================================================================
    // ERROR 4: multiple moves of same value
    // ===========================================================================
    // HINT: v moved to v2, can't move again to v3. Clone first or use references.
    let v = vec![1, 2, 3];
    let v2 = &v;
    let v3 = &v; // ERROR: v already moved to v2

    // ===========================================================================
    // ERROR 5: returning owned value that was borrowed
    // ===========================================================================
    // HINT: The String is moved into v, can't return &str. Return owned String.
    // let result = create_string();
    // println!("result = {result}"); 

    // ===========================================================================
    // ERROR 6: borrow after move
    // ===========================================================================
    // HINT: First borrow r, then move ownership. Either borrow first OR move first.
    let data = String::from("test");
    let r = &data;
    let data2 = data; // OK in NLL, but careful with lifetime of r
    println!("r = {data2}"); // r borrows original data
}

fn print_it(s: &mut String) {
    println!("inside: {s}");
}