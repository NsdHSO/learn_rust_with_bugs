//! 40 Borrowing Error Exercises - Each exercise has ONE error!
//!
//! Run `cargo check -p borrowing` to see all errors.
//! Each function demonstrates a different borrowing error.

fn main() {
    ex01(); ex02(); ex03(); ex04(); ex05();
    ex06(); ex07(); ex08(); ex09(); ex10();
    ex11(); ex12(); ex13(); ex14(); ex15();
    ex16(); ex17(); ex18(); ex19(); ex20();
    ex21(); ex22(); ex23(); ex24(); ex25();
    ex26(); ex27(); ex28(); ex29(); ex30();
    ex31(); ex32(); ex33(); ex34(); ex35();
    ex36(); ex37(); ex38(); ex39(); ex40();
    println!("40 errors - compile to see them!");
}

// ===== ERRORS 1-10 =====

// ERROR 1: E0499 - two mutable borrows
fn ex01() { let mut s = String::from("hello"); let r1 = &mut s; let r2 = &mut s; }

// ERROR 2: E0502 - mutable while immutable exists
fn ex02() { let mut t = String::from("world"); let r3 = &t; t.push_str("!"); }

// ERROR 3: E0596 - mutate through shared reference
fn ex03() { let mut u = String::from("immutable"); let r6 = &u; r6.push_str(" nope"); }

// ERROR 4: E0505 - borrow after move
fn ex04() { let v = vec![1,2,3]; let first = &v[0]; let v2 = v; println!("{}", first); }

// ERROR 5: E0502 - freeze
fn ex05() { let mut x = 10; let r = &x; let m = &mut x; }

// ERROR 6: E0502 - iterator invalidation
fn ex06() { let mut nums = vec![1,2,3]; for n in &nums { nums.push(10); } }

// ERROR 7: E0499 - overlapping mutable slices
fn ex07() { let mut arr = [1,2,3,4,5]; let a = &mut arr[0..3]; let b = &mut arr[2..5]; }

// ERROR 8: E0597 - match return borrow escaping
fn ex08() { let opt = Some(String::from("hi")); let result = match opt { Some(s) => s.as_str(), None => "", }; }

// ERROR 9: E0502 - NLL confusion
fn ex09() { let mut n = String::from("nll"); let r = &n; println!("{}", r); n.push_str("x"); }

// ERROR 10: E0509 - partial move in struct
fn ex10() { #[derive(Debug)] struct P { name: String } let p = P { name: String::from("A") }; let n = &p.name; let _m = p; }

// ===== ERRORS 11-20 =====

// ERROR 11: E0507 - dereference &Box
fn ex11() { let val = Box::new(5); let ref_val = &val; let _d = *ref_val; }

// ERROR 12: E0502 - mutate while iterating
fn ex12() { let mut items = vec![1,2,3]; for i in &items { items.push(4); } }

// ERROR 13: E0106 - missing lifetime
fn ex13() { fn f(s: &String) -> &str { s.as_str() } let s = String::from("x"); let _r = f(&s); }

// ERROR 14: E0502 - index then push
fn ex14() { let mut v = vec![1]; let r = &v[0]; v.push(2); }

// ERROR 15: E0505 - move while borrowed
fn ex15() { let v = vec![1,2,3]; let b = &v[0]; let _m = v; }

// ERROR 16: E0502 - mutable after borrow
fn ex16() { let v = vec![1]; let r = &v; v.push(2); }

// ERROR 17: E0106 - return local reference
fn ex17() { fn make_ref() -> &String { let l = String::from("l"); &l } }

// ERROR 18: E0596 - reborrow &
fn ex18() { let mut v = String::from("v"); let r = &v; let mr = &mut *r; }

// ERROR 19: E0597 - match borrowed return
fn ex19() { let o = Some(String::from("x")); let r = match o { Some(s) => s.as_str(), None=>"", }; }

// ERROR 20: E0502 - tuple field mutate
fn ex20() { let mut t = (1, 2); let r = &t.0; t.0 = 3; }

// ===== ERRORS 21-30 =====

// ERROR 21: E0502 - struct field freeze
fn ex21() { struct H { i: i32 } let mut h = H { i: 1 }; let r = &h.i; h.i = 2; }

// ERROR 22: E0502 - RefCell double borrow
fn ex22() { use std::cell::RefCell; let c = RefCell::new(vec![1]); let b = c.borrow(); let _bm = c.borrow_mut(); }

// ERROR 23: E0502 - closure capture mutate
fn ex23() { let c = String::from("c"); let mut v = vec![]; let f = || v.push(&c); f(); c.push_str("!"); }

// ERROR 24: E0502 - for loop mutate
fn ex24() { let mut l = vec![1,2,3]; for i in &l { l.push(4); } }

// ERROR 25: E0502 - while let mutate
fn ex25() { let mut o = Some(5); while let Some(ref mut n) = o { o = None; } }

// ERROR 26: E0509 - if let move
fn ex26() { let o: Option<String> = Some(String::from("x")); if let Some(s) = o { let _c = o; } }

// ERROR 27: E0507 - Box dereference
fn ex27() { let b = Box::new(5); let r = &b; let _d = *r; }

// ERROR 28: E0502 - vec index then push
fn ex28() { let mut v = vec![1]; let i = &v[0]; v.push(2); }

// ERROR 29: E0499 - slice overlap
fn ex29() { let mut a = [1,2,3,4,5]; let s = &a[..3]; let s2 = &mut a[2..]; }

// ERROR 30: E0507 - double dereference
fn ex30() { let v = 5; let r = &v; let rr = &r; let _d = **rr; }

// ===== ERRORS 31-40 =====

// ERROR 31: E0373 - thread borrow
fn ex31() { use std::thread; let d = String::from("d"); let h = thread::spawn(|| { let _x = &d; }); }

// ERROR 32: E0373 - thread borrow
fn ex32() { use std::thread; let d = String::from("d"); let h = thread::spawn(|| { println!("{}", d); }); }

// ERROR 33: E0502 - thread iterate after move
fn ex33() { use std::thread; let v = vec![1]; let h = thread::spawn(move || { for x in &v { println!("{}", x); }); }

// ERROR 34: E0505 - multiple borrow then move
fn ex34() { let o = Some(String::from("a")); let r1 = &o; let r2 = &o; let _m = o; }

// ERROR 35: E0502 - RefCell double
fn ex35() { use std::cell::RefCell; let c = RefCell::new(vec![1]); let b = c.borrow(); let bm = c.borrow_mut(); }

// ERROR 36: E0502 - nested RefCell
fn ex36() { use std::cell::RefCell; let o = RefCell::new(RefCell::new(5)); let b = o.borrow(); let i = b.borrow(); let _im = o.borrow_mut(); }

// ERROR 37: E0505 - loop borrow then move
fn ex37() { let v = vec![1,2]; for i in 0..v.len() { let x = &v[i]; let _m = v; } }

// ERROR 38: E0597 - return from closure
fn ex38() { let x = String::from("x"); let f = || -> &str { &x }; }

// ERROR 39: E0502 - mutate after borrow
fn ex39() { let mut s = String::from("s"); let r = &s; s.push_str("x"); }

// ERROR 40: E0106 - no lifetime
fn ex40() { struct S<'a> { x: &'a str } let _s = S { x: get() }; }

fn get() -> &str { "x" }