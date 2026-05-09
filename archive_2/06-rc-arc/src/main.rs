//! Fix all Rc & Arc errors so this crate compiles.
//! 
//! Learn: Reference counting, weak refs, memory leaks, thread safety
//!
//! Each error has ERROR comment and HINT.

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // ===========================================================================
    // ERROR 1: Rc used across threads
    // ===========================================================================
    // HINT: Rc is not Send. Use Arc for thread-safe ref count.
    use std::thread;
    let data = Rc::new(vec![1, 2, 3]);
    thread::spawn(move || {
        println!("{:?}", data); // ERROR: Rc not Send
    });

    // ===========================================================================
    // ERROR 2: Rc clone vs move
    // ===========================================================================
    // HINT: Rc clone increments ref count, doesn't move. Both stay valid.
    let a = Rc::new(String::from("hi"));
    let b = a.clone();
    println!("a = {}, b = {}", a, b); // Both valid

    // ===========================================================================
    // ERROR 3: cyclical references (memory leak)
    // ===========================================================================
    // HINT: Use Weak to break cycles.
    struct Node { next: Option<Rc<RefCell<Node>> }
    let a = Rc::new(RefCell::new(Node { next: None }));
    let b = Rc::new(RefCell::new(Node { next: Some(a.clone()) }));
    a.borrow_mut().next = Some(b.clone()); // LEAK: circular ref

    // ===========================================================================
    // ERROR 4: Weak upgrade after drop
    // ===========================================================================
    // HINT: Check upgrade() returns Option.
    use std::rc::Weak;
    let strong = Rc::new(1);
    let weak = Rc::downgrade(&strong);
    drop(strong);
    let upgraded = weak.upgrade(); // None!

    // ===========================================================================
    // ERROR 5: Arc<Vec<T>> not mutable
    // ===========================================================================
    // HINT: Arc gives shared access. Use Arc<Mutex<T>> for writes.
    use std::sync::Arc;
    use std::thread;
    let data = Arc::new(vec![1, 2, 3]);
    for _ in 0..3 {
        let d = data.clone();
        thread::spawn(move || {
            d.push(4); // ERROR: no &mut access
        });
    }

    // ===========================================================================
    // ERROR 6: Rc<RefCell<T>> across threads
    // ===========================================================================
    // HINT: RefCell not Send/Sync. Use Mutex.
    let data = Rc::new(RefCell::new(1));
    let d = data.clone();
    thread::spawn(move || {
        *d.borrow_mut() = 2; // ERROR: RefCell not Send
    });
}