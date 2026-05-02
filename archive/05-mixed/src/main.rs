//! Mixed exercise: fix ownership, borrowing, slice, and lifetime errors.

fn main() {
    let mut book = String::from("The Rust Programming Language");

    // ERROR 1: immutable borrow used after mutable borrow
    // HINT: &book (immutable) is held while book.push_str (mutable) runs.
    // Either do all mutation before creating chapter, or clone book first.
    let chapter = &mut book;
    chapter.push_str(" - Chapter 1");
    println!("reading: {chapter}");

    // ERROR 2: ownership lost, then reused
    // HINT: co_author = author moves ownership. print_author(&author) fails.
    // Borrow with &author, or clone: author.clone().
    let author = String::from("Steve Klabnik");
    let co_author = &author;
    print_author(&author);
    println!("co-author: {co_author}");

    // ERROR 3: dangling slice from local String
    // HINT: `copy` is a local variable dropped at end of first_word.
    // Operate on `s` directly (it's already a reference) instead of cloning.
    let first = first_word(&book);
    println!("first word: {first}");

    // ERROR 4: returning reference to local data
    // HINT: `headline` is local to summarize. Return an owned String instead.
    let summary = summarize(&book);
    println!("summary: {summary}");
}

fn print_author(name: &String) {
    println!("author: {name}");
}

fn first_word(s: &String) -> String {
    let copy = s.clone();
    let bytes = copy.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return copy[..i].to_string();
        }
    }
    copy[..].to_string()
}

fn summarize(text: &str) -> String{
    let headline = format!("Summary: {text}");
    headline[..8].to_string()
}
