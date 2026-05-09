//! Fix all Unsafe Rust errors so this crate compiles.
//! 
//! Learn: Raw pointers, transmute, pointer provenance, UB
//!
//! Each error has ERROR comment and HINT.

fn main() {
    // ===========================================================================
    // ERROR 1: null pointer dereference
    // ===========================================================================
    // HINT: Null pointer deref is UB. Check for null first.
    let p: *const i32 = std::ptr::null();
    unsafe {
        println!("{}", *p); // UB!
    }

    // ===========================================================================
    // ERROR 2: dangling pointer
    // ===========================================================================
    // HINT: Pointer beyond allocation is UB.
    let arr = [1, 2, 3];
    let p = arr.as_ptr().offset(10);
    unsafe {
        println!("{}", *p); // UB!
    }

    // ===========================================================================
    // ERROR 3: transmute invalid value
    // ===========================================================================
    // HINT: transmute must preserve valid bit patterns.
    let bits: u32 = 0xFFFFFFFF;
    let float: f32 = unsafe { std::mem::transmute(bits) }; // May be NaN

    // ===========================================================================
    // ERROR 4: pointer provenance
    // ===========================================================================
    // HINT: Must create pointer from & reference, not copy address.
    let mut n = 5;
    let p1 = &mut n as *mut i32;
    let p2 = p1;
    unsafe { *p2 = 10; } // UB!

    // ===========================================================================
    // ERROR 5: creating invalid bool
    // ===========================================================================
    // HINT: Only valid bit patterns for the type.
    let b: bool = unsafe { std::mem::transmute(2u8) }; // UB!

    // ===========================================================================
    // ERROR 6: unaligned pointer
    // ===========================================================================
    // HINT: Some types require alignment. Use align_to.
}