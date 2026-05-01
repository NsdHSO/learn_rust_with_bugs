//! Mixed exercise: fix ownership, borrowing, slice, and lifetime errors.

fn main() {
    let mut book = String::from("The Rust Programming Language");

    // ERROR 1: immutable borrow used after mutable borrow
    let chapter = &book;
    book.push_str(" - Chapter 1");
    println!("reading: {chapter}");

    // ERROR 2: ownership lost, then reused
    let author = String::from("Steve Klabnik");
    let co_author = author;
    print_author(&author);
    println!("co-author: {co_author}");

    // ERROR 3: dangling slice from local String
    let first = first_word(&book);
    println!("first word: {first}");

    // ERROR 4: returning reference to local data
    let summary = summarize(&book);
    println!("summary: {summary}");
}

fn print_author(name: &String) {
    println!("author: {name}");
}

fn first_word(s: &String) -> &str {
    let copy = s.clone();
    let bytes = copy.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &copy[..i];
        }
    }
    &copy[..]
}

fn summarize(text: &str) -> &str {
    let headline = format!("Summary: {text}");
    &headline[..8]
}
