//! Fix all lifetimes deep dive errors so this crate compiles.
//! 
//! Learn: Lifetime elision, HRTB, 'static, struct refs
//!
//! Each error has ERROR comment and HINT on how to fix.

fn main() {
    // ===========================================================================
    // ERROR 1: struct with reference field
    // ===========================================================================
    // HINT: References in structs need lifetime parameters.
    struct Person {
        name: &str, // ERROR: needs lifetime
        age: &i32, // ERROR: needs lifetime
    }
    let name = "John";
    let age = 30;
    let p = Person { name, age };

    // ===========================================================================
    // ERROR 2: returning reference to local
    // ===========================================================================
    // HINT: Local variable drops at function end. Return owned or 'static.
    fn create() -> &str { // ERROR: missing lifetime
        let s = "hello";
        s // ERROR: s drops at end
    }

    // ===========================================================================
    // ERROR 3: lifetime elision failure
    // ===========================================================================
    // HINT: Two inputs, one output - compiler can't infer output lifetime.
    fn first_word(s: &str) -> &str { // ERROR: ambiguous
        let pos = s.find(' ').unwrap_or(s.len());
        &s[..pos]
    }

    // ===========================================================================
    // ERROR 4: impl missing lifetime
    // ===========================================================================
    // HINT: impl block must declare lifetime parameters.
    struct Config<'a> { value: &'a str }
    impl Config { // ERROR: missing <'a>
        fn get(&self) -> &str { self.value }
    }

    // ===========================================================================
    // ERROR 5: self-referential struct
    // ===========================================================================
    // HINT: Can't have &self in struct. Use Box or index.
    struct Node {
        value: i32,
        next: Option<&Node>, // ERROR: self-ref not allowed
    }

    // ===========================================================================
    // ERROR 6: 'static confusion
    // ===========================================================================
    // HINT: 'static means entire program, not just "long-lived".
    fn get_static() -> &'static str {
        let s = get_input(); // ERROR: not 'static
        s
    }
    fn get_input() -> String { String::from("hi") }

    // ===========================================================================
    // ERROR 7: multiple lifetime parameters
    // ===========================================================================
    // HINT: Need to specify which lifetime for output.
    fn either<'a, 'b>(x: &'a str, y: &'b str) -> &str { // ERROR: which lifetime?
        if x.len() > y.len() { x } else { y }
    }
}