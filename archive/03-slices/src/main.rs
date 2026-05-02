//! Fix all slice errors so this crate compiles.

fn main() {
    // ERROR 1: function missing lifetime annotation for returned slice
    // HINT: The compiler cannot prove the returned &str lives as long as the inputs.
    // Add an explicit lifetime: fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
    let s1 = String::from("hello world");
    let s2 = String::from("foo bar");
    let result = longest(&s1, &s2);
    println!("longest: {result}");

    // ERROR 2: returning a slice that refers to a local String
    // HINT: `s` is dropped when get_word returns, so &s would dangle.
    // Return an owned String instead of &str.
    let bad = get_word();
    println!("word: {bad}");

    // ERROR 3: array slice out of bounds (runtime panic, not compile error)
    // Leave it as a warning comment; uncomment to see the panic.
    // HINT: Use safe slicing: a.get(0..10) returns Option<&[T]> instead of panicking.
    // let a = [1, 2, 3];
    // let _slice = &a[0..10];
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

fn get_word() -> &str {
    let s = String::from("hello");
    &s[..]
}
