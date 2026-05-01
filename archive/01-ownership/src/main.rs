//! Fix all ownership errors so this crate compiles.

fn main() {
    // ERROR 1: use after move
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s1 = {s1}, s2 = {s2}");

    // ERROR 2: use after passing ownership to function
    let s3 = String::from("world");
    print_it(s3);
    println!("after: {s3}");

    // ERROR 3: tuple destructuring loses ownership tracking
    let s4 = String::from("foo");
    let s5 = String::from("bar");
    let result = combine(s4, s5);
    println!("combined: {result}, plus original s4 = {s4}");
}

fn print_it(s: String) {
    // ERROR 4: wrong variable name
    println!("inside function: {u}");
}

fn combine(a: String, b: String) -> String {
    format!("{a}-{b}")
}
