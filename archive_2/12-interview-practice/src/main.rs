//! Interview Practice - Study guide and compilation errors.
//! 
//! Practice answering these questions AND fix the errors in this file.
//! Each error represents a common interview question.

//! ===========================================================================
//! Q1: What makes Something Clone but not Copy?
//! ===========================================================================
//! HINT: Clone is explicit deep copy, Copy is implicit bitwise copy.

//! ERROR 1: Using clone when copy would suffice
struct MyInt(i32);
impl Clone for MyInt {
    fn clone(&self) -> Self { MyInt(self.0) }
}
fn copy_example() {
    let x = MyInt(5);
    let y = x.clone(); // HINT: MyInt doesn't implement Copy
}

//! ===========================================================================
//! Q2: & vs &mut?
//! ===========================================================================
//! HINT: & is shared/read-only, &mut is exclusive/write.

//! ERROR 2: Multiple &mut borrows
fn borrow_example() {
    let mut s = String::from("hi");
    let r1 = &mut s;
    let r2 = &mut s; // ERROR
}

//! ===========================================================================
//! Q3: Why no multiple &mut?
//! ===========================================================================
//! HINT: Aliasing XOR mutability - prevents data races.

//! ERROR 3: Mutable borrow while immutable exists
fn freeze_example() {
    let mut v = vec![1, 2];
    let r = &v;
    v.push(3); // ERROR
}

//! ===========================================================================
//! Q4: What is a lifetime?
//! ===========================================================================
//! HINT: Tracks how long references are valid.

//! ERROR 4: Missing lifetime on struct
struct PersonRef {
    name: &str, // ERROR
}

//! ===========================================================================
//! Q5: What is borrow checker?
//! ===========================================================================
//! HINT: Compile-time analyzer enforcing borrowing rules.

//! ERROR 5: Borrow after move
fn move_after_borrow() {
    let s = String::from("hi");
    let r = &s;
    let s2 = s;
    println!("{r}"); // ERROR: r borrows moved s
}

//! ===========================================================================
//! Q6: Rc vs Arc?
//! ===========================================================================
//! HINT: Rc = single-threaded, Arc = multi-threaded (atomic).

use std::rc::Rc;
fn rc_thread_example() {
    use std::thread;
    let r = Rc::new(1);
    thread::spawn(move || { // ERROR: Rc not Send
        println!("{}", r);
    });
}

//! ===========================================================================
//! Q7: Cell vs RefCell?
//! ===========================================================================
//! HINT: Cell for Copy types, RefCell for runtime borrow.

//! ERROR 7: Cell with non-Copy
use std::cell::Cell;
fn cell_example() {
    let c = Cell::new(String::from("hi")); // ERROR: String not Copy
}

//! ===========================================================================
//! Q8: Send vs Sync?
//! ===========================================================================
//! HINT: Send = can transfer, Sync = can share reference.

//! ERROR 8: RefCell not Sync
use std::cell::RefCell;
fn sync_example() {
    // let r: &Sync won't work with RefCell
}

//! ===========================================================================
//! Q9: dyn Trait vs impl Trait?
//! ===========================================================================
//! HINT: dyn = dynamic dispatch, impl = static dispatch.

//! ERROR 9: Return type ambiguity
fn return_trait() -> &dyn std::fmt::Display { // OK but verbose
    &"hi"
}

//! ===========================================================================
//! Q10: Trait object?
//! ===========================================================================
//! HINT: Box<dyn Trait> or &dyn Trait for polymorphism.

trait MyTrait {
    fn call(&self);
}
fn obj_example() {
    // let t: MyTrait = ...; // ERROR: can't use dyn without Box
}

fn main() {
    println!("Run each --package to see errors and fix them!");
    println!("Study each error and understand WHY it fails.");
}