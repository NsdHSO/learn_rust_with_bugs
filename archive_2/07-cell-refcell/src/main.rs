//! Fix all Cell & RefCell errors so this crate compiles.
//! 
//! Learn: Interior mutability, runtime borrow checking, Copy vs Clone
//!
//! Each error has ERROR comment and HINT.

use std::cell::{Cell, RefCell};

fn main() {
    // ===========================================================================
    // ERROR 1: Cell with non-Copy type
    // ===========================================================================
    // HINT: Cell requires Copy. Use RefCell for non-Copy types.
    let c = Cell::new(String::from("hi")); // ERROR: String not Copy

    // ===========================================================================
    // ERROR 2: RefCell double borrow
    // ===========================================================================
    // HINT: Can't borrow mutably while immutably borrowed.
    let r = RefCell::new(vec![1, 2, 3]);
    let m = r.borrow_mut();
    let b = r.borrow(); // PANIC: already mutably borrowed

    // ===========================================================================
    // ERROR 3: RefCell multiple mutable borrows
    // ===========================================================================
    let r2 = RefCell::new(5);
    let a = r2.borrow_mut();
    let b = r2.borrow_mut(); // PANIC

    // ===========================================================================
    // ERROR 4: RefCell in thread
    // ===========================================================================
    // HINT: RefCell is not Send. Use Mutex instead.
    use std::thread;
    let data = RefCell::new(vec![1]);
    thread::spawn(|| {
        println!("{:?}", data.borrow()); // ERROR: not Send
    });

    // ===========================================================================
    // ERROR 5: Cell vs RefCell usage
    // ===========================================================================
    // HINT: Cell<T>: get()/set() for Copy. RefCell: borrow()/borrow_mut().
    let cell = Cell::new(5);
    cell.set(10); // OK for Copy

    // ===========================================================================
    // ERROR 6: Rc<RefCell<T>> pattern
    // ===========================================================================
    // HINT: Allows shared mutability but not thread-safe.
    use std::rc::Rc;
    let data = Rc::new(RefCell::new(vec![1, 2, 3]));
    data.borrow_mut().push(4); // OK for single-threaded
}