//! Fix all ownership errors so this crate compiles.

fn main() {
    // ERROR 1: use after move
    // HINT: s1 was moved into s2. Either borrow with &s1 instead of moving,
    // or clone s1 (s1.clone()) if you need two owned copies.
    let s1 = String::from("hello");
    let s2 = &s1;
    println!("s1 = {s1}, s2 = {s2}");

    // ERROR 2: use after passing ownership to function
    // HINT: print_it takes ownership of s3. Pass a reference (&s3)
    // or clone s3, or change print_it to borrow and return the value.
    let s3 = String::from("world");
    print_it(&s3);
    println!("after: {s3}");

    // ERROR 3: tuple destructuring loses ownership tracking
    // HINT: combine takes ownership of s4. Borrow (&s4) if you still need s4 later.
    let s4 = String::from("foo");
    let s5 = String::from("bar");
    let result = combine(&s4, s5);
    println!("combined: {result}, plus original s4 = {s4}");
}

fn print_it(s: &String) {
    // ERROR 4: wrong variable name
    // HINT: the parameter is named `s`, not `u`. Fix the format string.
    println!("inside function: {s}");
}

fn combine(a: &String, b: String) -> String {
    format!("{a}-{b}")
}
