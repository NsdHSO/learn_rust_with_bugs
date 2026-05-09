//! Fix all slices deep dive errors so this crate compiles.
//! 
//! Learn: Fat pointers, UTF-8, byte vs char indexing, bounds
//!
//! Each error has ERROR comment and HINT on how to fix.

fn main() {
    // ===========================================================================
    // ERROR 1: slice with invalid UTF-8 boundary
    // ===========================================================================
    // HINT: &str must be valid UTF-8. Don't corrupt internal bytes.
    let mut s = String::from("hello");
    unsafe {
        let bytes = s.as_bytes_mut();
        bytes[2] = 0x80; // corrupt UTF-8
    }
    let slice = &s[0..3]; // PANIC: invalid UTF-8!

    // ===========================================================================
    // ERROR 2: index string with integer
    // ===========================================================================
    // HINT: Use chars().nth() or bytes().nth() for character indexing.
    let s = "héllo";
    let c = s[2]; // ERROR: can't index &str with usize

    // ===========================================================================
    // ERROR 3: slice out of bounds
    // ===========================================================================
    // HINT: Use get() to return Option, or ensure bounds are valid.
    let v = vec![1, 2, 3];
    let slice = &v[0..10]; // PANIC: out of bounds

    // ===========================================================================
    // ERROR 4: &str passed where String expected
    // ===========================================================================
    // HINT: &str is borrowed, String is owned. Convert with .to_string().
    fn print_string(s: String) { println!("{s}"); }
    let x = "hello";
    print_string(x); // ERROR: need String, got &str

    // ===========================================================================
    // ERROR 5: mutable slice aliasing
    // ===========================================================================
    // HINT: Can't have overlapping mutable slices.
    let mut v = vec![1, 2, 3, 4, 5];
    let a = &mut v[0..2];
    let b = &mut v[2..4]; // ERROR: may overlap internally

    // ===========================================================================
    // ERROR 6: returning borrowed slice
    // ===========================================================================
    // HINT: Need lifetime to connect input to output.
    // fn slice_at(s: &str, i: usize) -> &str { &s[i..] }
}