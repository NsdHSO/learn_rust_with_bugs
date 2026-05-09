//! Fix all Mutex & Threading errors so this crate compiles.
//! 
//! Learn: Send, Sync, Mutex, deadlocks, Arc
//!
//! Each error has ERROR comment and HINT.

fn main() {
    // ===========================================================================
    // ERROR 1: Rc not Send
    // ===========================================================================
    // HINT: Rc uses non-atomic ref count. Use Arc.
    use std::rc::Rc;
    use std::thread;
    let data = Rc::new(vec![1, 2, 3]);
    thread::spawn(move || {
        println!("{:?}", data); // ERROR: Rc not Send
    });

    // ===========================================================================
    // ERROR 2: RefCell not Sync
    // ===========================================================================
    // HINT: RefCell has runtime borrow tracking, not thread-safe.
    use std::cell::RefCell;
    use std::thread;
    let data = RefCell::new(vec![1, 2, 3]);
    let handle = thread::spawn(|| {
        let r = &data; // ERROR: RefCell not Sync
        println!("{:?}", r.borrow());
    });

    // ===========================================================================
    // ERROR 3: Arc without Mutex for writes
    // ===========================================================================
    // HINT: Arc gives shared read. Use Arc<Mutex<T>> for writes.
    use std::sync::Arc;
    use std::thread;
    let data = Arc::new(vec![1, 2, 3]);
    for _ in 0..3 {
        let d = data.clone();
        thread::spawn(move || {
            d.push(4); // ERROR: no &mut
        });
    }

    // ===========================================================================
    // ERROR 4: Deadlock with multiple mutexes
    // ===========================================================================
    use std::sync::Mutex;
    let a = Mutex::new(1);
    let b = Mutex::new(2);
    // Thread 1: lock a, then b
    // Thread 2: lock b, then a
    // DEADLOCK!

    // ===========================================================================
    // ERROR 5: Mutex poisoning
    // ===========================================================================
    // HINT: If thread panics, mutex is poisoned.
    let m = Mutex::new(5);
    {
        let mut g = m.lock().unwrap();
        *g = 10;
    }

    // ===========================================================================
    // ERROR 6: RwLock for read-heavy
    // ===========================================================================
    // HINT: RwLock allows many readers, exclusive writer.
    use std::sync::RwLock;
    let rw = RwLock::new(5);
    let r = rw.read().unwrap(); // many readers OK

    // ===========================================================================
    // ERROR 7: scoped threads
    // ===========================================================================
    // HINT: std::thread::scope (Rust 1.63+) for scoped spawning.
    // thread::scope(|s| { s.spawn(|| { ... }); });
}