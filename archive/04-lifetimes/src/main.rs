//! Fix all lifetime errors so this crate compiles.

fn main() {
    // ERROR 1: struct holding a reference needs a lifetime parameter
    // HINT: Any struct holding a reference must declare a lifetime:
    // struct Excerpt<'a> { part: &'a str }
    let text = String::from("hello world");
    let excerpt = Excerpt { part: &text[..5] };
    println!("excerpt: {}", excerpt.part);

    // ERROR 2: function returning reference without lifetime annotation
    // HINT: When a function returns a reference and takes references as input,
    // add lifetimes to tie them together: fn pick_longer<'a>(x: &'a str, y: &'a str) -> &'a str
    let s1 = "short";
    let s2 = "longer";
    let result = pick_longer(s1, s2);
    println!("longer: {result}");

    // ERROR 3: method with mismatched lifetime elision
    // HINT: The impl block must match the struct's lifetime parameter:
    // impl<'a> Holder<'a> { ... }
    let holder = Holder { value: &text };
    let announced = holder.announce("news");
    println!("announced: {announced}");
}

// Missing lifetime parameter on struct
struct Excerpt {
    part: &str,
}

// Missing lifetime annotation
fn pick_longer(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

struct Holder<'a> {
    value: &'a str,
}

impl Holder {
    fn announce(&self, msg: &str) -> &str {
        println!("Announcement: {msg}");
        self.value
    }
}
